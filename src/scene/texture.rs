use super::loader::Asset;
use crate::context::Context;
use std::sync::Arc;
use vulkano::buffer::BufferContents;
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, PrimaryCommandBufferAbstract,
};
use vulkano::format::Format;
use vulkano::image::view::ImageView;
use vulkano::image::{ImageDimensions, ImmutableImage, MipmapsCount};
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::sampler::Sampler;
use vulkano::sync;
use vulkano::sync::GpuFuture;

pub struct Texture {
    pub image_view: Arc<ImageView<ImmutableImage>>,
    pub sampler: Arc<Sampler>,
}

impl Texture {
    pub fn from_gltf_image(
        image_data: gltf::image::Data,
        sampler: Arc<Sampler>,
        context: &Context,
    ) -> Arc<Texture> {
        Self::new(
            image_data.pixels,
            image_data.width,
            image_data.height,
            gltf_image_format_to_vulkan_format(&image_data.format),
            sampler,
            context,
        )
    }

    pub fn new_one_by_one(sampler: Arc<Sampler>, context: &Context) -> Arc<Texture> {
        Self::new(
            vec![1.0, 1.0, 1.0],
            1,
            1,
            Format::R8G8B8_UNORM,
            sampler,
            context,
        )
    }

    pub fn new<I, Px>(
        data_iterator: I,
        width: u32,
        height: u32,
        format: Format,
        sampler: Arc<Sampler>,
        context: &Context,
    ) -> Arc<Texture>
    where
        [Px]: BufferContents,
        I: IntoIterator<Item = Px>,
        I::IntoIter: ExactSizeIterator,
    {
        let future = sync::now(context.device()).boxed();

        let command_buffer_allocator =
            StandardCommandBufferAllocator::new(context.device(), Default::default());

        let memory_allocator = StandardMemoryAllocator::new_default(context.device());

        let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
            &command_buffer_allocator,
            context.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        let texture = {
            let dimensions = ImageDimensions::Dim2d {
                width,
                height,
                array_layers: 1,
            };

            let image = ImmutableImage::from_iter(
                &memory_allocator,
                data_iterator,
                dimensions,
                MipmapsCount::One,
                format,
                &mut command_buffer_builder,
            )
            .unwrap();

            ImageView::new_default(image).unwrap()
        };

        let command_buffer = command_buffer_builder.build().unwrap();

        let future = future
            .then_execute(context.queue(), command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();

        future.wait(None).unwrap();

        Arc::new(Texture {
            image_view: texture,
            sampler,
        })
    }
}

fn gltf_image_format_to_vulkan_format(format: &gltf::image::Format) -> Format {
    match format {
        gltf::image::Format::R8 => Format::R8_UINT,
        gltf::image::Format::R8G8 => Format::R8G8_UINT,
        gltf::image::Format::R8G8B8 => Format::R8G8B8_UINT,
        gltf::image::Format::R8G8B8A8 => Format::R8G8B8A8_UINT,
        gltf::image::Format::R16 => Format::R16_UINT,
        gltf::image::Format::R16G16 => Format::R16G16_UINT,
        gltf::image::Format::R16G16B16 => Format::R16G16B16_UINT,
        gltf::image::Format::R16G16B16A16 => Format::R16G16B16A16_UINT,
        gltf::image::Format::R32G32B32FLOAT => Format::R32G32B32_SFLOAT,
        gltf::image::Format::R32G32B32A32FLOAT => Format::R32G32B32A32_SFLOAT,
    }
}

impl Asset for Texture {}
