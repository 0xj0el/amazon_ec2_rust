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
        BurstBuilder {
            descritpors: Vec::new(),
        }
    }
}

impl BurstBuilder{
    pub fn create_set(&mut self, description: MachineSet){}

    pub fn run()
}

//explore more or closures
// builder pattern -> https://blog.ediri.io/design-patterns-in-rust-an-introduction-to-the-builder-pattern
fn main(){
    let mut b = BurstBuilder::default();
    b.add_set("server", 1, MachineSetup::new("t2.micro", "ami-e18aa9b", |ssh|{
        ssh.exec("sudo yum install htop")
    })); //t2.micro is amazons instance type , id -ami, 

    b.add_set("client", 2, MachineSetup::new("t2.micro", "ami-e18aa9b", |ssh|{
        ssh.exec("sudo yum install htop")
    }));

    b.run(|vms: HashMap<String, MachineSet>|{
        let server_ip = vms["server"][0].ip;
        vms["client"].for_each_parllel(|client| {
            client.exec(cmd);
        })

    });

}