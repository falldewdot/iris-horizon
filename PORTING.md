# Informe de portado: Iris

## Resumen
```
Iris  (iris 1.11.7+mc26.3)
Cargador: Fabric
Clases Java: 983
Esfuerzo de portado estimado: ALTO (puntaje 188.7)
Clase(s) de entrada: no detectada (revisa fabric.mod.json / entrypoints)
Bloques: 0  Items: 0  Entidades: 0

```

## Codigo
```
[DIFICIL] Block entities / maquinas: 26 clases
          Estado por bloque e inventarios (API experimental).
          - net.irisshaders.iris.mixin.fabric.MixinExtendedBlockModelSubmit
          - net.irisshaders.iris.layer.BlockEntityRenderStateShard
          - net.irisshaders.iris.mixin.MixinTheEndPortalRenderer
          - net.irisshaders.iris.mixin.entity_render_context.BlockModelRenderStateMixin
          - net.irisshaders.iris.mixin.entity_render_context.ItemStackStateLayerMixin

[DIFICIL] Entidades y mobs: 6 clases
          IA y sincronizacion de entidades.
          - net.irisshaders.iris.mixin.MixinGameRenderer_NightVisionCompat
          - net.irisshaders.iris.mixin.MixinLightTexture
          - net.irisshaders.iris.mixin.entity_render_context.MixinEntityRenderDispatcher
          - net.irisshaders.iris.pathways.HandRenderer
          - net.irisshaders.iris.uniforms.CommonUniforms

[DIFICIL] Pantallas y menus (GUI): 20 clases
          Interfaces de contenedor: reescritura completa.
          - net.irisshaders.iris.compat.modmenu.ModMenuIntegration
          - net.irisshaders.iris.Iris
          - net.irisshaders.iris.IrisVKOnly
          - net.irisshaders.iris.apiimpl.IrisApiV0Impl
          - net.irisshaders.iris.compat.sodium.config.IrisConfig

[DIFICIL] Mixins / coremods: 190 clases
          Parchean Java Minecraft, que en Horizon no existe.
          - net.irisshaders.iris.mixin.fabric.MixinExtendedBlockModelFeatureRenderer
          - net.irisshaders.iris.mixin.fabric.MixinExtendedBlockModelSubmit
          - net.irisshaders.iris.mixin.fabric.MixinFluidRendererImpl
          - net.irisshaders.iris.mixin.fabric.MixinRenderTypes
          - net.irisshaders.iris.compat.dh.DHMixinConfigPlugin

[PARCIAL] Registro de bloques/items: 10 clases
          Depende del soporte de contenido de la API.
          - net.irisshaders.iris.api.v0.item.IrisItemLightProvider
          - net.irisshaders.iris.mixin.entity_render_context.ItemStackStateLayerMixin
          - net.irisshaders.iris.mixin.entity_render_context.MixinElytraLayer
          - net.irisshaders.iris.mixin.entity_render_context.MixinEntityRenderDispatcher
          - net.irisshaders.iris.mixin.entity_render_context.MixinEquipmentLayerRenderer

[PARCIAL] HUD y render: 164 clases
          UI simple si; render propio depende de la API.
          - net.irisshaders.iris.mixin.fabric.MixinExtendedBlockModelSubmit
          - net.irisshaders.iris.Iris
          - net.irisshaders.iris.compat.dh.DHCompatInternal
          - net.irisshaders.iris.compat.dh.IrisGenericRenderProgram
          - net.irisshaders.iris.compat.dh.IrisLodRenderProgram

```

## Datos
```
Datos reutilizables sin tocar codigo (se copian al exportar):
  Texturas              4
  Modelos/blockstates   0
  Idiomas               29
  Sonidos               0
  Recetas               0
  Loot tables           0
  Tags                  0
  Logros                0
  Worldgen JSON         0
```

## API de Minecraft usada
```
Clases de Minecraft mas referenciadas (hay que buscar su equivalente en Horizon):
     76 net.minecraft.client.Minecraft
     34 net.minecraft.resources.Identifier
     28 net.minecraft.network.chat.Component
     27 net.minecraft.client.renderer.GameRenderer
     26 net.minecraft.client.gui.GuiGraphicsExtractor
     20 net.minecraft.client.gui.Gui
     20 net.minecraft.client.Options
     19 net.minecraft.network.chat.MutableComponent
     17 net.minecraft.client.OptionInstance
     13 net.minecraft.client.multiplayer.ClientLevel
     13 net.minecraft.client.Camera
     13 net.minecraft.client.renderer.texture.AbstractTexture
     12 net.minecraft.ChatFormatting
     12 net.minecraft.client.renderer.RenderPipelines
     11 net.minecraft.client.gui.Font
     11 net.minecraft.client.gui.navigation.ScreenRectangle
     11 net.minecraft.util.Mth
     10 net.minecraft.client.gui.screens.Screen
      9 net.minecraft.client.player.LocalPlayer
      9 net.minecraft.client.input.MouseButtonEvent
      9 net.minecraft.client.input.KeyEvent
      9 net.minecraft.client.renderer.texture.SpriteContents
      8 net.minecraft.client.gui.components.AbstractSelectionList$Entry
      8 net.minecraft.client.gui.components.AbstractSelectionList
      8 net.minecraft.core.DefaultedRegistry

APIs del cargador (se descartan al portar):
      1 net.fabricmc.fabric.api.client.keymapping.v1.KeyMappingHelper
      1 net.fabricmc.loader.api.Version
      1 net.fabricmc.loader.api.SemanticVersion
      1 net.fabricmc.loader.api.ModContainer
      1 net.fabricmc.loader.api.VersionParsingException
      1 net.fabricmc.loader.api.metadata.ModMetadata
      1 net.fabricmc.loader.api.FabricLoader
```
