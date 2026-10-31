// Datos detectados en el mod original. No esta conectado a ningun hook: usalo como referencia.
// Port de Iris (Fabric) a Horizon. Generado por Jar2Horizon: punto de partida, no traduccion.
// TODO: declara los hooks en horizon-mixin.toml y mueve aqui la logica (docs de Horizon: Make a mod).

pub const MOD_ID: &str = "iris";
pub const MOD_NAME: &str = "Iris";

/// (id, nombre) detectados en los archivos de idioma/modelos del original.
pub const BLOCKS: &[(&str, &str)] = &[
];

/// (id, nombre) detectados en los archivos de idioma/modelos del original.
pub const ITEMS: &[(&str, &str)] = &[
];

/// (id, nombre) detectados en los archivos de idioma/modelos del original.
pub const ENTITIES: &[(&str, &str)] = &[
];

pub fn on_load() {
    // TODO [PARCIAL] Registro de bloques/items: Depende del soporte de contenido de la API.
    //   Java: net.irisshaders.iris.api.v0.item.IrisItemLightProvider
    //   Java: net.irisshaders.iris.mixin.entity_render_context.ItemStackStateLayerMixin
    //   Java: net.irisshaders.iris.mixin.entity_render_context.MixinElytraLayer
    //   Java: net.irisshaders.iris.mixin.entity_render_context.MixinEntityRenderDispatcher
    // TODO [PARCIAL] HUD y render: UI simple si; render propio depende de la API.
    //   Java: net.irisshaders.iris.mixin.fabric.MixinExtendedBlockModelSubmit
    //   Java: net.irisshaders.iris.Iris
    //   Java: net.irisshaders.iris.compat.dh.DHCompatInternal
    //   Java: net.irisshaders.iris.compat.dh.IrisGenericRenderProgram
    // TODO [DIFICIL] Block entities / maquinas: Estado por bloque e inventarios (API experimental).
    //   Java: net.irisshaders.iris.mixin.fabric.MixinExtendedBlockModelSubmit
    //   Java: net.irisshaders.iris.layer.BlockEntityRenderStateShard
    //   Java: net.irisshaders.iris.mixin.MixinTheEndPortalRenderer
    //   Java: net.irisshaders.iris.mixin.entity_render_context.BlockModelRenderStateMixin
    // TODO [DIFICIL] Entidades y mobs: IA y sincronizacion de entidades.
    //   Java: net.irisshaders.iris.mixin.MixinGameRenderer_NightVisionCompat
    //   Java: net.irisshaders.iris.mixin.MixinLightTexture
    //   Java: net.irisshaders.iris.mixin.entity_render_context.MixinEntityRenderDispatcher
    //   Java: net.irisshaders.iris.pathways.HandRenderer
    // TODO [DIFICIL] Pantallas y menus (GUI): Interfaces de contenedor: reescritura completa.
    //   Java: net.irisshaders.iris.compat.modmenu.ModMenuIntegration
    //   Java: net.irisshaders.iris.Iris
    //   Java: net.irisshaders.iris.IrisVKOnly
    //   Java: net.irisshaders.iris.apiimpl.IrisApiV0Impl
    // TODO [DIFICIL] Mixins / coremods: Parchean Java Minecraft, que en Horizon no existe.
    //   Java: net.irisshaders.iris.mixin.fabric.MixinExtendedBlockModelFeatureRenderer
    //   Java: net.irisshaders.iris.mixin.fabric.MixinExtendedBlockModelSubmit
    //   Java: net.irisshaders.iris.mixin.fabric.MixinFluidRendererImpl
    //   Java: net.irisshaders.iris.mixin.fabric.MixinRenderTypes
}
