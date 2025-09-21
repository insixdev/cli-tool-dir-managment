use std::env::{self, args, current_dir};
use std::fs::{create_dir_all, File, OpenOptions, Permissions};
use std::io::{self, BufRead, Error, Read, Write };
use std::num::ParseIntError;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
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
fn print_init(shell: &str) {
   match shell {
      "zsh" | "bash" => {
         println!(
         r#"cdc() {{
    target=$(/mnt/data/proyect/rust/cliToolCarpet "$@")
    cd "$target" || return 1
}}
cdc_list() {{
    target=$(/mnt/data/proyect/rust/cliToolCarpet -l | fzf | awk '{{print $2}}')
    sudo cd "$target" || return 1
}}"#);
      }  _ => eprintln!("Shell no soportada"),
   }
}
fn warn(exit: bool){
   println!("escribe -h para mas info ");
   if exit {
      std::process::exit(1); } 
} 
fn option_help(){
   println!("la configuracion de directorios esta guardado en: $HOME/.local/share/clitool")

}

fn get_data_dir() -> PathBuf {
    // (o el directorio de configuración, dependiendo del uso).
   match directories::ProjectDirs::from("com", "insixdev", "cliTool") {
      Some(proj_dirs) => {
         let data_dir = proj_dirs.data_dir(); // para linux seria
         // $HOME/.local/share/clitool

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

   if args.get(1).map(|s| s.as_str()) == Some("init") {
        if let Some(shell) = args.get(2) {
            print_init(shell);
            return Ok(());
      }
   }

   if !(args.iter().nth(4) == None) {println!("opcion desconocida/max arg exedido"); std::process::exit(1)} 
   let arg1 = get_arg(&args, 1);
   let arg2 = get_arg(&args, 2);
   let arg3 = get_arg(&args, 3);

   match arg1{
      "-c" => option_create_dir(&arg2, &arg3 ),
      "-g" => option_go(&arg2),
      "-m" => print!("fmodoif"),
      "-l" => option_list(), 
      "-d" => option_delete_dir(&arg2),
      "-h" => option_help(),
      _ =>  warn(true),
   }
   Ok(())
}
fn add_one(number: String) -> String{
   let number = number.parse::<i32>();
   let number = match number {
      Ok(num) => num+1,
      Err(par) => {println!("{} : intente devuelta, ", par); process::exit(1);}
   };
   number.to_string()
}

fn option_go(number: &str){
   let file = open_file();
   let file = match file {
      Ok(f) => f,
      Err(er) => {println!("hubo un error al procesar el archivo: {}", er); std::process::exit(1)},
   };

   let reader = io::BufReader::new(file);
   let line_count = reader.lines();

   let mut final_dir: String = String::new();

   for (i,lim) in line_count.enumerate() {
      let st = add_one(i.to_string());
      if number == st {
         final_dir = match lim {
            Ok(li) => li,
            Err(err) => {println!("error {} ", err);process::exit(1);},
         };
         break;
      }
   }

   if !final_dir.is_empty() {

      let new_dir = Path::new(&final_dir);

      if let Err(e) = env::set_current_dir(&new_dir) {
         eprintln!("Error al cambiar de directorio: {}", e);
      } else {
         println!("{}", final_dir);
         std::process::exit(1);
      }
   }else {
      println!("no se encontro el directorio ")
   }

}
/// en listar las opciones
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
/// fn que verifica si esta correcto el pathBuf
fn verify_dir(dir_arg: &str ) -> PathBuf{
   let dir: PathBuf; 
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
   dir

}

fn option_create_dir(dir_arg: &str , name_arg: &str) {
   let dir = verify_dir(&dir_arg);

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

