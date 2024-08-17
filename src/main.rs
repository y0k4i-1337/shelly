use clap::{Parser, ValueEnum};
use clap_num::maybe_hex_range;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
//use base64::prelude::*;
//use base85;

fn key_in_range(s: &str) -> Result<u8, String> {
    maybe_hex_range(s, 0x00, 0xff)
}

/// Shelly - Shellcode encryption for fun and profit
#[derive(Debug, Parser)]
#[command(name = "shelly")]
#[command(bin_name = "shelly")]
#[command(about = "Shellcode encryption for fun and profit", long_about = None)]
//#[command(arg_required_else_help(true))]
struct Cli {
    /// Path to the input file. Use "-" for stdin.
    #[arg(short, long)]
    input: Option<PathBuf>,
    /// Apply ROT cipher with the specified shift value.
    #[arg(short, long, default_value_t = 0)]
    rot: i8,
    /// Apply XOR cipher with the specified key value.
    #[arg(short, long, value_parser=key_in_range ,default_value = "0x00")]
    xor: u8,
    /// Output format for the encrypted shellcode.
    #[clap(value_enum, short, long, default_value_t = Format::Csharp)]
    format: Format,
    /// Name of the variable to store the encrypted shellcode.
    #[arg(short, long, default_value = "buf")]
    var: String,
    ///// Encoding to use for the encrypted shellcode.
    //#[clap(value_enum, short, long, default_value_t = Encoding::None)]
    //encoding: Encoding,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Encoding {
    None,
    Hex,
    Base64,
    Base85,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Format {
    Csharp,
    Python,
    C,
    Psh,
    Vba,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let input_path = cli.input;
    let rot_shift = cli.rot;
    let xor_key: u8 = cli.xor;
    let format = cli.format;
    let output_var = cli.var;
    //let encoding = cli.encoding;
    let mut input_data = Vec::new();


    if let Some(input_path) = input_path {
        if input_path == PathBuf::from("-") {
            io::stdin().read_to_end(&mut input_data)?;
        } else {
            let mut file = File::open(input_path)?;
            file.read_to_end(&mut input_data)?;
        }
    } else {
        eprintln!("No input file specified");
        return Ok(());
    }

    if rot_shift != 0 {
        input_data = apply_rot_cipher(&input_data, rot_shift);
    }

    if xor_key > 0 {
        input_data = apply_xor_cipher(&input_data, xor_key);
    }

    // Output the encrypted shellcode according to the specified format
    match format {
        Format::Csharp => print_csharp_code(&input_data, &output_var),
        Format::Python => print_python_code(&input_data, &output_var),
        Format::C => print_c_code(&input_data, &output_var),
        Format::Psh => print_powershell_code(&input_data, &output_var),
        Format::Vba => print_vba_code(&input_data, &output_var),
    }

    print_decryption_commands(format, &output_var, -rot_shift, xor_key);


    // Encoding and output
    // match encoding {
    //     Encoding::Hex => {
    //         let hex_data = hex::encode(&input_data);
    //         print_encoded_payload(&hex_data, format, &output_var, encoding);
    //         //print_decryption_commands(&input_data, format, &output_var, encoding, -rot_shift, xor_key);
    //     },
    //     Encoding::Base64 => {
    //         let base64_data = BASE64_STANDARD.encode(&input_data);
    //         print_encoded_payload(&base64_data, format, &output_var, encoding);
    //         //print_decryption_commands(&input_data, format, &output_var, encoding, -rot_shift, xor_key);
    //     },
    //     Encoding::Base85 => {
    //         let base85_data = base85::encode(&input_data);
    //         print_encoded_payload(&base85_data, format, &output_var, encoding);
    //         //print_decryption_commands(&input_data, format, &output_var, encoding, -rot_shift, xor_key);
    //     },
    //     Encoding::None => {
    //         match format {
    //             Format::Csharp => print_csharp_code(&input_data, &output_var),
    //             Format::Python => print_python_code(&input_data, &output_var),
    //             Format::C => print_c_code(&input_data, &output_var),
    //             Format::Psh => print_powershell_code(&input_data, &output_var),
    //             Format::Vba => print_vba_code(&input_data, &output_var),
    //         }
    //         //print_decryption_commands(&input_data, format, &output_var, encoding, -rot_shift, xor_key);
    //     },
    // }

    Ok(())
}

/// Applies ROT cipher transformation to the data
fn apply_rot_cipher(data: &[u8], shift: i8) -> Vec<u8> {
    let shift = shift as i16; // Convert to i16 for calculation
    let normalized_shift = ((shift % 256) + 256) % 256; // Normalize to 0-255 range

    data.iter()
        .map(|&byte| {
            let shifted = (byte as i16 + normalized_shift) % 256;
            shifted as u8
        })
        .collect()
}


/// Applies XOR cipher transformation to the data
fn apply_xor_cipher(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|&byte| byte ^ key).collect()
}


/// Print C# code
fn print_csharp_code(data: &[u8], output_var: &str) {
    print!("byte[] {} = new byte[{}] {{", output_var, data.len());
    for (i, chunk) in data.chunks(12).enumerate() {
        if i > 0 {
            print!(",");
        }
        print!("\n    ");
        let hex_chunk: Vec<String> = chunk.iter().map(|b| format!("0x{:02x}", b)).collect();
        print!("{}", hex_chunk.join(", "));
    }
    println!("\n}};");
}

/// Print Python code
fn print_python_code(data: &[u8], output_var: &str) {
    print!("{} = [", output_var);
    for (i, chunk) in data.chunks(12).enumerate() {
        if i > 0 {
            print!(",");
        }
        print!("\n    ");
        let hex_chunk: Vec<String> = chunk.iter().map(|b| format!("0x{:02x}", b)).collect();
        print!("{}", hex_chunk.join(", "));
    }
    println!("\n]");
}

/// Print C code
fn print_c_code(data: &[u8], output_var: &str) {
    print!("unsigned char {}[{}] = {{", output_var, data.len());
    for (i, chunk) in data.chunks(12).enumerate() {
        if i > 0 {
            print!(",");
        }
        print!("\n    ");
        let hex_chunk: Vec<String> = chunk.iter().map(|b| format!("0x{:02x}", b)).collect();
        print!("{}", hex_chunk.join(", "));
    }
    println!("\n}};");
}

/// Print PowerShell code
fn print_powershell_code(data: &[u8], output_var: &str) {
    print!("${} = [byte[]] @(", output_var);
    for (i, chunk) in data.chunks(12).enumerate() {
        if i > 0 {
            print!(",");
        }
        print!("\n    ");
        let hex_chunk: Vec<String> = chunk.iter().map(|b| format!("0x{:02x}", b)).collect();
        print!("{}", hex_chunk.join(", "));
    }
    println!("\n)");
}

/// Print VBA code
fn print_vba_code(data: &[u8], output_var: &str) {
    print!("Dim {}({}) As Byte = Array(", output_var, data.len() - 1);
    for (i, chunk) in data.chunks(12).enumerate() {
        if i > 0 {
            print!(",");
        }
        print!("\n    ");
        let hex_chunk: Vec<String> = chunk.iter().map(|b| format!("&H{:02x}", b)).collect();
        print!("{}", hex_chunk.join(", "));
    }
    println!("\n)");
}

/// Print encoded payload
/// TODO: print encoded payload in proper format
// fn print_encoded_payload(data: &str, format: Format, output_var: &str, encoding: Encoding) {
//     // Generate code to declare the encoded data variable
//     match encoding {
//         Encoding::Hex => match format {
//             Format::Csharp => {
//                 let hex_data = split_long_string(data, 80);
//                 println!("string {}_str = @\"{}\";", output_var, hex_data);
//                 println!("byte[] {} = Convert.FromHexString({}_str);", output_var, output_var);
//             }
//             Format::Python => {
//                 let hex_data = split_long_string(data, 80);
//                 println!("{} = bytes.fromhex(\"{}\");", output_var, hex_data);
//             }
//             Format::C => {
//                 let hex_data = split_long_string(data, 80);
//                 println!("unsigned char {}[] = {{", output_var);
//                 for line in hex_data.lines() {
//                     println!("    {};", line);
//                 }
//                 println!("}};");
//             }
//             Format::Psh => {
//                 let hex_data = split_long_string(data, 80);
//                 println!("${} = [System.Text.Encoding]::ASCII.GetString([System.Convert]::FromHexString(\"{}\"));", output_var, hex_data);
//             }
//             Format::Vba => {
//                 let hex_data = split_long_string(data, 80);
//                 println!("Dim {}() As Byte = HexToByte(\"{}\");", output_var, hex_data);
//             }
//         },
//         Encoding::Base64 => match format {
//             Format::Csharp => {
//                 let base64_data = split_long_string(data, 80);
//                 println!("string {}_str = @\"{}\";", output_var, base64_data);
//                 println!("byte[] {} = Convert.FromBase64String({}_str);", output_var, output_var);
//             }
//             Format::Python => {
//                 let base64_data = split_long_string(data, 80);
//                 println!("{} = base64.b64decode(\"{}\");", output_var, base64_data);
//             }
//             Format::C => {
//                 let base64_data = split_long_string(data, 80);
//                 println!("unsigned char {}[] = {{", output_var);
//                 for line in base64_data.lines() {
//                     println!("    {};", line);
//                 }
//                 println!("}};");
//             }
//             Format::Psh => {
//                 let base64_data = split_long_string(data, 80);
//                 println!("${} = [System.Text.Encoding]::ASCII.GetString([System.Convert]::FromBase64String(\"{}\"));", output_var, base64_data);
//             }
//             Format::Vba => {
//                 let base64_data = split_long_string(data, 80);
//                 println!("Dim {}() As Byte = Base64ToByte(\"{}\");", output_var, base64_data);
//             }
//         },
//         Encoding::Base85 => match format {
//             Format::Csharp => {
//                 let base85_data = split_long_string(data, 80);
//                 println!("string {}_str = @\"{}\";", output_var, base85_data);
//                 println!("byte[] {} = Base85Decode({}_str);", output_var, output_var);
//             }
//             Format::Python => {
//                 let base85_data = split_long_string(data, 80);
//                 println!("{} = base85.b85decode(\"{}\");", output_var, base85_data);
//             }
//             Format::C => {
//                 let base85_data = split_long_string(data, 80);
//                 println!("unsigned char {}[] = {{", output_var);
//                 for line in base85_data.lines() {
//                     println!("    {};", line);
//                 }
//                 println!("}};");
//             }
//             Format::Psh => {
//                 let base85_data = split_long_string(data, 80);
//                 println!("${} = [System.Text.Encoding]::ASCII.GetString([System.Text.Encoding]::Base85.Decode(\"{}\"));", output_var, base85_data);
//             }
//             Format::Vba => {
//                 let base85_data = split_long_string(data, 80);
//                 println!("Dim {}() As Byte = Base85ToByte(\"{}\");", output_var, base85_data);
//             }
//         },
//         Encoding::None =>  {

//         }
//     }
// }

/// Print decryption commands
fn print_decryption_commands(format: Format, output_var: &str, rot_key: i8, xor_key: u8) {
    println!("\n// Decrypt commands:");

    match format {
        Format::Csharp => {
            print!("// C# code to decrypt:\n");
            println!("byte[] {} = /* payload bytes here */;", output_var);
            println!("{} = Enumerable.Range(0, {}.Length)", output_var, output_var);
            println!("    .Select(i => (byte)({}[i] ^ {:#02x}))", output_var, xor_key);
            println!("    .ToArray();");
            println!("{} = ROT({},{})", output_var, output_var, rot_key);
            println!("\n// ROT function in C#:");
            println!("public static byte[] ROT(byte[] data, int shift) {{");
            println!("    // Ensure the result is non-negative before applying % 256");
            println!("    return data.Select(b => (byte)((b + shift + 256) % 256)).ToArray();");
            println!("}}");
        },
        Format::Python => {
            print!("### Python code to decrypt:\n");
            println!("{} = bytearray(/* payload bytes here */)", output_var);
            println!("{} = [byte ^ 0x{:02x} for byte in {}]", output_var, xor_key, output_var);
            println!("{} = ROT({}, {})", output_var, output_var, rot_key);
            println!("\n# ROT function in Python:");
            println!("def ROT(data, shift):");
            println!("    # Ensure the result is non-negative before applying % 256");
            println!("    return bytearray((byte + shift + 256) % 256 for byte in data)");
        },
        Format::C => {
            print!("/* C code to decrypt: */\n");
            println!("// ROT function in C:");
            println!("unsigned char ROT(unsigned char byte, int shift) {{");
            println!("    // Ensure the result is non-negative before applying % 256");
            println!("    return (byte + shift + 256) % 256;");
            println!("}}\n");
            println!("\n// main function in C:");
            println!("int main(int argc, char **argv) {{");
            println!("    for (size_t i = 0; i < sizeof({}); i++) {{", output_var);
            println!("        {}[i] ^= 0x{:02x}; // XOR decrypt", output_var, xor_key);
            println!("    }}");
            println!("    for (size_t i = 0; i < sizeof({}); i++) {{", output_var);
            println!("        {}[i] = ROT({}[i], {}); // ROT decrypt", output_var, output_var, rot_key);
            println!("    }}\n");
            println!("    int (*ret)() = (int(*)()){};", output_var);
            println!("    ret();");
            println!("}}");

        },
        Format::Psh => {
            print!("### PowerShell code to decrypt:\n");
            println!("${} = [byte[]]@(/* payload bytes here */)", output_var);
            println!("${} = ${{}}.ToCharArray() | ForEach-Object {{ [byte]($_ -bxor 0x{:02x}) }}", output_var, xor_key);
            println!("${} = ROT(${},{})", output_var, output_var, rot_key);
            println!("\n# ROT function in PowerShell:");
            println!("function ROT($data, $shift) {{");
            //println!("    $shift = $shift % 256;");
            println!("    # Ensure the result is non-negative before applying % 256");
            println!("    return $data | ForEach-Object {{ ($_ + $shift +256) % 256 }};");
            println!("}}");
        },
        Format::Vba => {
            print!("### VBA code to decrypt:\n");
            println!("Dim {}() As Byte = Array(/* payload bytes here */)", output_var);
            println!("For i = LBound({}) To UBound({})", output_var, output_var);
            println!("    {}(i) = {}(i) Xor {} ' XOR decrypt", output_var, output_var, xor_key);
            println!("Next i");
            println!("{} = ROT({},{})", output_var, output_var, rot_key);
            println!("\n' ROT function in VBA:");
            println!("Function ROT(data() As Byte, shift As Integer) As Byte()");
            println!("    Dim i As Long");
            println!("    Dim result() As Byte");
            println!("    ReDim result(LBound(data) To UBound(data))");
            println!("    For i = LBound(data) To UBound(data)");
            println!("        ' Ensure the result is non-negative before applying Mod 256");
            println!("        result(i) = (data(i) + shift +256) Mod 256");
            println!("    Next i");
            println!("    ROT = result");
            println!("End Function");
        },
    }
}


// Helper function to split data into multiple lines
// fn split_long_string(data: &str, max_line_length: usize) -> String {
//     data.chars()
//         .collect::<Vec<_>>()
//         .chunks(max_line_length)
//         .map(|chunk| chunk.iter().collect::<String>())
//         .collect::<Vec<_>>()
//         .join("\n")
// }
