use anyhow::Result;
use psd::Psd;

#[test]
fn visibility() -> Result<()> {
    let psd = include_bytes!("./fixtures/visibility.psd");
    let psd = Psd::from_bytes(psd)?;

    let layers = psd.layers();
    layers.iter().for_each(|layer| {
        match layer.name() {
            "visible" => assert!(layer.visible()),
            "invisible" => assert!(!layer.visible()),
            _ => (),
        }
    });
    Ok(())
}

#[test]
fn visibility_group() -> Result<()> {
    let psd = include_bytes!("./fixtures/visibility_group.psd");
    let psd = Psd::from_bytes(psd)?;

    let layers = psd.layers();
    layers.iter().for_each(|layer| {
        match layer.name() {
            "visible" => assert_eq!(layer.visible(), true),
            "invisible" => assert_eq!(layer.visible(), false),
            _ => (),
        }
    });
    let groups = psd.groups();
    groups.iter().for_each(|group| {
        match group.1.name() {
            "visible" => assert_eq!(group.1.visible(), true, "testing group {}", group.0),
            "invisible" => assert_eq!(group.1.visible(), false, "testing group {}", group.0),
            _ => (),
        }
    });
    Ok(())
}
