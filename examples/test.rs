use log::{info, LevelFilter};
use vmrest_rs::{VMParameter, VMPowerOperation, VMRestContext};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new().init()?;
    let context = VMRestContext::new("user", "Ganzgeheim1!");
    for vm in context.get_vms().await? {
        info!("{vm:?}");
        let state = context.get_vm_power_state(&vm.id, None).await?;
        info!("{state:?}");
        if vm.path.contains("Windows XP") {
            let settings = context.get_vm_settings(&vm.id, None).await?;
            let mut parameters = VMParameter {
                processors: settings.cpu.unwrap().processors,
                memory: settings.memory.unwrap(),
            };
            parameters.processors += 12;
            // let clone = context.create_vm_clone(&vm.id, "Goofy XP VM 2", None).await?;
            
            // if let Some(clone_vm) = context.get_all_vms().await?.iter().filter(|vm| vm.id == clone.id).collect::<Vec<_>>().first() {
            //     context.create_vm_registration("Goofy XP VM 2", &clone_vm.path, None).await?;
            // }
            // context.set_vm_settings(&clone.id, &parameters, None).await?;
            // context.set_vm_power_state(&vm.id, VMPowerOperation::On, None).await?;
        }
    }
    info!("{:#?}", context.get_vmnets().await?);
    Ok(())
}