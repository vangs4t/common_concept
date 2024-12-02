use std::io::stdout;

fn main() {
    println!("Hello, world!");
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

    println!("Ini adalah tipe data integer signed");
    println!("signed {signed_1}, {signed_2}");
    println!("ini adalah tipe data integer unsigned");
    println!("unsigned {unsigned_1}");
}
// contoh scalar tipe float
/*Tipe data float dapat menerima semua angka decimal
 */
#[test]
fn tipe_data_float(){

}