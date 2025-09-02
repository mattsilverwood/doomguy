use doomguy::wad::Wad;

fn main() {
    let _wad = Wad::open("./freedoom2.wad").expect("Something broke");

    dbg!(_wad);
}
