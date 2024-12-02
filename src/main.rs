use violin_rs::block::Block;
use violin_rs::block::block_registry::BlockTexture;
use violin_rs::block::component::BlockDestructibleByMiningComponent;
use violin_rs::image::Image;
use violin_rs::pack::{Pack, ScriptData};
use violin_rs::vio::{Buildable, Identifier, SemVer};

fn main() {
    let pack_id = "constructione";
    let mut pack = Pack::new(
        "Constructione",
        "constructione",
        "NaKeR",
        SemVer::new(0, 0, 1),
        "Building is not boring.",
        r"C:\Users\narol\AppData\Local\Packages\Microsoft.MinecraftUWP_8wekyb3d8bbwe\LocalState\games\com.mojang\development_behavior_packs",
        r"C:\Users\narol\AppData\Local\Packages\Microsoft.MinecraftUWP_8wekyb3d8bbwe\LocalState\games\com.mojang\development_resource_packs",
        Image::new("./textures/cherry_planks_base.png"),
        ScriptData::new(
          SemVer::new_beta(1, 14, 0),
          SemVer::new_beta(1, 3, 0),
          "./scripts"
        )
    );

    pack.register_block_texture(BlockTexture::new(
       Image::new("./textures/cherry_planks_base.png").compose(
           Image::new("./textures/wood_overlays/vertical_sheets.png"),
           violin_rs::image::blend_modes::BlendMode::Overlay
       ),
       Identifier::new(pack_id, "cherry_vertical_sheets"),
       "cherry_sheets"
    ));
    pack.register_block_texture(BlockTexture::new(
        Image::new("./textures/cherry_planks_base.png").compose(
            Image::new("./textures/wood_overlays/box.png"),
            violin_rs::image::blend_modes::BlendMode::Overlay
        ),
        Identifier::new(pack_id, "cherry_box"),
        "cherry_box"
    ));

    pack.register_block(Block::new(
       Identifier::new("idk", "idk")
    ).using_components(vec![
        BlockDestructibleByMiningComponent::new(5.0).build()
    ]));

    pack.generate();
}
