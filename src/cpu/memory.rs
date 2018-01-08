type Address = u16;

trait Memory {
    type Storage;

    fn fetch(&Address) -> u8;
    fn store(&Address, &u8);

}