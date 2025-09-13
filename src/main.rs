use std::env::{args, current_dir};
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{self, BufRead, Error, Write };
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::process;

struct Directory {
   directory: PathBuf,
   name: String,
}

impl Directory{
   fn new(path:PathBuf, name: String) -> Directory{
      let dir = Directory {name: name, directory: path};
      dir
   }
}

fn warn(exit: bool){
   println!("escribe -h para mas info ");
   if exit {
      std::process::exit(1); } } fn option_help(){

}
fn get_data_dir() -> PathBuf {
    // (o el directorio de configuración, dependiendo del uso).
   match directories::ProjectDirs::from("com", "insixdev", "cliTool") {
      Some(proj_dirs) => {
         let data_dir = proj_dirs.data_dir(); // para linux seria
         // $HOME/.local/share/cli-tool-dir-managment

         if !data_dir.exists() { // si no existe 
            create_dir_all(data_dir).expect("Failed to create data directory.");
         }
         return data_dir.to_path_buf();
      }
      _ => (),
   }
   // Fallback en caso de que no se pueda determinar el directorio
   // (esto es poco probable, pero es una buena práctica).
   PathBuf::from("./data")
}


fn get_arg(args: &Vec<String>, val: i32) -> &str {

   let mut arg_f: &str = " "; // espacio por defecto
   if let Some(arg) = args.iter().nth(val as usize) {
      arg_f = arg;
   }
   arg_f
}
fn main() -> io::Result<()>{

   let args: Vec<String> = args().collect();
   if !(args.iter().nth(4) == None) {println!("opcion desconocida/max arg exedido"); std::process::exit(1)} 
   let arg1 = get_arg(&args, 1);
   let arg2 = get_arg(&args, 2);
   let arg3 = get_arg(&args, 3);

   match arg1{
      "-c" => option_create_dir(&arg2, &arg3 ),
      "-m" => print!("fmodoif"),
      "-l" => option_list(), 
      "-d" => option_delete_dir(&arg2),
      "-h" => option_help(),
      _ =>  warn(true),
   }
   Ok(())
}

fn option_list(){
   let file = open_file();
   // verficamos si hubo un error refactoring de una funcion aqui
   let file = match file {
      Ok(f) => f,
      Err(err) => {println!("mierda no se abrio tu archiv: razon: {} ", err); process::exit(1)}
   };
   let reader = io::BufReader::new(file);
// Contar líneas
   let line_count = reader.lines();
   for (i,lin) in line_count.enumerate() {
      match lin {
         Ok(f) => println!("dir {}: {}", i+1, f),
         Err(err) => println!("Error al procesar la linea. Err: {}", err),
      }
   }
}
/// primer arg para list o no 
fn option_delete_dir(arg: &str) {
   if arg.trim().is_empty(){warn(true);}
   match arg {
      "l" => {
         print!("puto")
      }
      _ => warn(true)
   }
}

fn option_create_dir(dir_arg: &str , name_arg: &str) {
   let mut dir: PathBuf = PathBuf::new();
   if dir_arg.trim().is_empty(){
      dir = current_dir().unwrap();
   } else {
      let path = PathBuf::from(&dir_arg);
      if !(path.exists() && path.is_dir()){
         println!("el directorio no es valido");
         std::process::exit(1);
      } else {
         dir = path;
      }


   }

   let dir_f: Directory = Directory::new(dir, "test".to_string());

   let file = open_file();
   let mut file_f = match file {
      Ok(f) => f,
      Err(er) => {println!("hubo un error al procesar el archivo: {}", er); std::process::exit(1)},
   };
   let tx: &[u8] = dir_f.directory.as_os_str().as_bytes();

   file_f.write_all(tx).unwrap();

   file_f.write_all(b"\n").unwrap(); // agregás el salto de línea manual
   println!("succes el dir: {:?} ha sido agregado con existo", dir_f.directory);

}

/// Fn que abre o crea el archivo dependiendo
/// de si ya se creo o no 
fn open_file() -> Result<File, io::Error>{
   // get the data directory of the user to append the dir.txt to create file 
   let data_dir = get_data_dir().join("dir.txt"); 
   let file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .append(true)
      .open(&data_dir);

   file
}

