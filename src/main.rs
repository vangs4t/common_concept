


fn main() {
    let _result = fungsi_return();
    let hasil  = without_return("Jhoni");
    println!("ini adalah return value {hasil}");
}

/*Function atau fungsi atau dalam bahasa java method
merupakan bagian dimana kita bisa membuat sebuah program dan menjalankan 
perintah2 */
// fn fungsi(){
//     println!("ini adalah fungsi");
// }
/*di setiap function pasti ada tanda kurung nah disana kita bisa menambahkan 
beberapa variable atau parameter */
// fn fn_with_parameters(angka :i32){
//     println!("Ini adalah angka {angka}");
// }
// contoh fungsi dengan beberapa parameter
// fn fn_with_many_parameters(kata: &str, decimal: f64, benar : bool){
//     println!("Fungsi dengan beberapa parameter");
//     println!("{kata},{decimal},{benar}");
// }
// statement adalah fungsi yang tidak mengembalikan apapun
// expresi fungsi yang mengembalikan value
#[test]
fn statement_function(){
    // statement merupakan variable yang tidak mengembalikan nilai yang berbeda
    // atau sudah di tentukan sebelumnya
    let this_statement = 6;
    let statement = "kanjut";

    println!("ini adalah statement {this_statement}");
    println!("ini juga sama {statement}");

    //sedangkan expresi merupakan perubahan nilai yang berbeda
    // dari yang sudah ditentukan sebelumnya
    // let this_expression = this_statement + 1;
    /*nilai yang di dalam blok merupakan expresi karna terjadi perubahan
    nilai value atau pengembalian nilai value yang sudah berubah */
    let expresi = {
        let result = 5;
        result + 1
    };
    println!("ini adalah expresi {}", expresi);

}
/*return function di dalam rust tidak seperti bahasa pemograman lain nya
yang secara gamblang di wajibkan menulis return 
di dalam rust kita hanya menambahkan tanda panah -> setelah itu baru menulis
kan mau mereturn apa
dan untuk melakukan return itu kita gausah pake semicollon atau fungsi tidak akan mengembalikan apapun */
fn fungsi_return() -> i32{
    let hasil = 4;
    println!("this is hasil {hasil}");
    return hasil;
}
// contoh fungsi dengan parameter dan tanpa return statement
fn without_return (_nama: &str) -> &str{
    "Jhoni"
}

/*Scalar Type
Scalar tipe merepresentasikan single value atau satu value
rust memiliki 4 tipe scalar utama yaitu integer, float, boolean, number 
*/
// contoh scalar tipe integer yaitu
/*Tipe data integer memiliki 2 bagian yaitu signed dan unsigned 
perbedaan nya dapat dilihat dari penggunaan kata kunci tipe data
lalu apa perbedaan nya? 
perbedaan paling mencolok adalah unsigned hanya menerima bilangan positif
sedangkan signed dapat menerima bilangan positif dan negatif
unsigned ditandai dengan huruf u sedangkan signed dengan huruf i*/
#[test]
fn tipe_data_integer(){
    let unsigned_1 : u32 = 4; // u32 merupakan unsigned
    let signed_1 : i32 = -14; // i32 signed
    let signed_2 : i32 = 13;

    // ini adalah integer decimal, namun decimal di rust integer literal
    // dipisahkan dengan underscore
    let decimal : u32 = 9_22;
    println!("This is decimal {decimal}");

    // bilangan hex yang sering kita temukan dalam penggunaaan warna 
    let hexadecimal = 0x2f;
    println!("this is hexadecimal {hexadecimal}");

    // bilangan octal merepresentasikan dalam penggunaan memori
    let octals = 0o22;
    println!("this is octal {octals}");

    // bilangan biner biasanya digunakan untuk merepresentasikan suatu data
    // di dalam bidang komputer
    let binnary = 0b1101_0000;
    println!("this is binnary {binnary}");

    println!("Ini adalah tipe data integer signed");
    println!("signed {signed_1}, {signed_2}");
    println!("ini adalah tipe data integer unsigned");
    println!("unsigned {unsigned_1}");
}
// contoh scalar tipe float
/*Tipe data float dapat menerima semua angka decimal 
decimal di dalam pemogramman menggunakan titik dan bukan koma
 */
#[test]
fn tipe_data_float(){
    let float_1 : f32 = 1.4;
    let float_2 : f32 = 2941.3221;

    println!("{float_1}, {float_2}");
}

/*Contoh penggunaan perhitungan pada pemograman rust
sebenarnya sama aja kaya bahasa pemograman lain*/
// const NUMBER1 : i32 = 11;
// const NUMBER2 : i32 = 5;
// fn perhitungan(){
//     // pernjumlahan
//     let penjumlahan = NUMBER1 + NUMBER2;
//     println!("Penjumlahan dari {NUMBER1} + {NUMBER2} = {penjumlahan}");

//     // pengurangan
//     let pengurangan = NUMBER1 - NUMBER2;
//     println!("Pengurangan dari 12 - 5 = {pengurangan}");

//     // perkalian
//     let perkalian = NUMBER1 * NUMBER2;
//     println!("Perkalian dari 12 * 5 = {perkalian}");

//     // pembagian 
//     let pembagian = NUMBER1 / NUMBER2;
//     println!("pembagian dari 12 / 5 = {pembagian}");

//     // modulus
//     let modulus = NUMBER1 % NUMBER2;
//     println!("Modulus dari 12 % 5 = {modulus}");
// }

// boolean hanya memiliki 2 fungsi yaitu true dan false
#[test]
fn nilai_boolean(){
    let x = true;

    let y : bool = false;

    println!("{x}");
    println!("{y}");
}

/*tuple merupakan himpunan dari beberapa type scalar, saat sudah dibuat
maka tidak bisa menambahkan type ke dalam himpunan atau pun mengurangi himpunan
untuk nemanbahkan tuple kita hanya perlu membuat tanda kurung setelah 
penambahan nama variable */
#[test]
fn tuples(){
    let himpunan : (i32, f64, char) = (321, 64.7, 's');
    
    // dan untuk pemanggilan index di dalam tuple kita bisa menggunakan 
    // nama variable.no index
    // index pertama di dalam tuple selalu dimulai dari 0

    let bilangan = himpunan.0;
    let decimal = himpunan.1;
    let charakter = himpunan.2;

    println!("tuple pertama = {}", bilangan);
    println!("tuple kedua = {}", decimal);
    println!("tuple ketiga = {}", charakter);
}

/*Array tipe hapir sama kaya tuple namun cuma menerima type scalar yang sama saja
 */
#[test]
fn arraystype() {
    let arr = [1,2,3,4,5];
    // untuk pemanggilan kita bisa menggunakan nama variable[]
    // arr juga sama index pertama dimulai dari 0
    // let one = arr[0];
    // let two = arr[1];

    for (i,x) in arr.iter().enumerate(){
        println!("{i}, {x}");
    }
}