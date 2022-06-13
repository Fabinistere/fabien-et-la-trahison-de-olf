use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
        render_resource::{
            std140::{AsStd140, Std140},
            *,
        },
        renderer::RenderDevice,
    },
    sprite::{Material2d, Material2dPipeline},
};

#[derive(TypeUuid, Clone)]
#[uuid = "bc0c5954-dbe6-45ce-bb79-fd6b10078c3d"]
pub struct CustomMaterial {
    pub brightness: f32,
    pub progress: f32,
    pub texture: Handle<Image>,
}

#[derive(AsStd140, Clone)]
pub struct CustomMaterialUniformData {
    pub brightness: f32,
    pub progress: f32,
}

pub struct GpuCustomMaterial {
    bind_group: BindGroup,
}

impl Material2d for CustomMaterial {
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/dialog_box_character_appear.wgsl"))
    }

    fn bind_group(material: &GpuCustomMaterial) -> &BindGroup {
        &material.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            CustomMaterialUniformData::std140_size_static() as u64,
                        ),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        view_dimension: TextureViewDimension::D2,
                        sample_type: TextureSampleType::Float { filterable: false },
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
        })
    }
}

impl RenderAsset for CustomMaterial {
    type ExtractedAsset = CustomMaterial;
    type PreparedAsset = GpuCustomMaterial;
    type Param = (
        SRes<RenderDevice>,
        SRes<RenderAssets<Image>>,
        SRes<Material2dPipeline<CustomMaterial>>,
    );

    fn extract_asset(&self) -> CustomMaterial {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: CustomMaterial,
        (render_device, gpu_images, pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<GpuCustomMaterial, PrepareAssetError<CustomMaterial>> {
        let uniform_data = CustomMaterialUniformData {
            brightness: extracted_asset.brightness,
            progress: extracted_asset.progress,
        };

        let (texture_view, sampler) = if let Some(result) = pipeline
            .mesh2d_pipeline
            .get_image_texture(gpu_images, &Some(extracted_asset.texture.clone()))
        {
            result
        } else {
            return Err(PrepareAssetError::RetryNextUpdate(extracted_asset));
        };

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: uniform_data.as_std140().as_bytes(),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &pipeline.material2d_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(texture_view),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(sampler),
                },
            ],
        });

        Ok(GpuCustomMaterial { bind_group })
    }
}
