use log::{info, LevelFilter};
use vmrest_rs::{CreateVmnetParameter, NetworkType, VMParameter, VMPowerOperation, VMRestContext};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new().with_level(LevelFilter::Info).init()?;
    let context = VMRestContext::new("user", "Ganzgeheim1!");
    for vm in context.get_vms().await? {
        info!("{vm:?}");
        let state = context.get_vm_power_state(&vm.id, None).await?;
        info!("{state:?}");
    }
    info!("{:#?}", context.get_vmnets().await?);
    Ok(())
}
