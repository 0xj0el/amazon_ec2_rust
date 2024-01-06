struct Burst{

}

struct MachineSet{

}
struct BurstBuilder {
    descritpors: Vec<MachineSet>
}

//Default is a rust trait that lets u construct something wihout arguments
impl Default from for BurstBuilder{
    fn default() -> Self {

    }
}
impl BurstBuilder{
    pub fn new() -> Self {}

    pub fn create_set(&mut self, description: MachineSet){}

    pub fn run()
}