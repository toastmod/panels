use std::time::{Duration, Instant};
use crate::bindgroupreg::BindGroupReg;
use crate::modelbuffers::Model;
use crate::renderobj::RenderObject;
use crate::resourcebytes::*;
use crate::texture;
use crate::texturerenderer::{TextureIndex, TextureRenderer};
use crate::timing::{CallStatus, Timing,};
use crate::transform2d::Transform2D;
use image::GenericImageView;
use wgpu::SurfaceTexture;
use wgpu::util::DeviceExt;
use winit::{event::*, window::*};
use crate::programhook::ProgramHook;
use crate::renderablestate::RenderableState;
use crate::util::fps_to_dur;

/// The render function for the WGPU `State`, defined by the user and called in the EventLoop
/// The `bool` parameter indicates a forced surface redraw request.
pub type StateRenderFunction = Fn(&mut State, bool) -> Result<(),wgpu::SurfaceError>;

pub struct State {
    // pub renderf: Box<StateRenderFunction>,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipelines: Vec<wgpu::RenderPipeline>,
    pub models: Vec<Model>,
    pub uniform_buffers: Vec<wgpu::Buffer>,
    pub panel_bind_group_layout: wgpu::BindGroupLayout,
    pub bind_groups: Vec<wgpu::BindGroup>,

    /// All textures stored in this state.
    pub textures: Vec<texture::Texture>,

    // The renderers for all textures, including the main Surface.
    // pub texture_renderers: Vec<TextureRenderer>,

    // Panels that will render.
    // pub panel_objects: Vec<RenderObject>,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        // texture setup
        let diffuse_texture = texture::Texture::from_bytes(&device, &queue, diffuse_bytes).unwrap();

        // uniform buffer setup

        let pos_unif = Transform2D {
            pos: [0.5, 0.5, 0.0],
        };

        let pos_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[pos_unif]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // 2D Panel Bindgroup setup
        let panel_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    // texture
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    // sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            filtering: true,
                            comparison: false,
                        },
                        count: None,
                    },
                    // uniform position
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        let main_panel_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &panel_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: pos_buffer.as_entire_binding(),
                },
            ],
        });

        // render pipeline setup
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&panel_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                clamp_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        // models setup
        let mut models = vec![];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(PENTAGON_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(PENTAGON_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = PENTAGON_INDICES.len() as u32;

        models.push(Model {
            vertex_buffer,
            index_buffer,
            offset_buffer: pos_buffer,
            num_indices,
        });

        Self {
            // renderf,
            surface,
            device,
            queue,
            config,
            size,
            render_pipelines: vec![render_pipeline],
            models,
            uniform_buffers: vec![],
            panel_bind_group_layout,
            bind_groups: vec![main_panel_bind_group],
            textures: vec![diffuse_texture],
            // texture_renderers: vec![],
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Resized(_) => {}
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested => {}
            WindowEvent::Destroyed => {}
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::ReceivedCharacter(_) => {}
            WindowEvent::Focused(_) => {}
            WindowEvent::KeyboardInput { .. } => {}
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::MouseInput { .. } => {}
            WindowEvent::TouchpadPressure { .. } => {}
            WindowEvent::AxisMotion { .. } => {}
            WindowEvent::Touch(_) => {}
            WindowEvent::ScaleFactorChanged { .. } => {}
            WindowEvent::ThemeChanged(_) => {}
        }
    }

    pub fn update(&mut self) {
        // update offsets
        // for obj in &self.objects {
        //     println!("update");
        //     self.queue.write_buffer(
        //         &self.models[obj.model].offset_buffer,
        //         0,
        //         bytemuck::cast_slice(&[obj.position]),
        //     )
        // }
    }

    /// Renders the Surface objects (panels).
    /// `force_all` will draw all panels.
    // pub fn render_surface(&mut self, force_all: bool, clear_or_load: wgpu::LoadOp) -> Result<(), wgpu::SurfaceError>{
    //
    //     // SURFACE TEXTURE RENDERPASSING
    //     // This is where Panels are drawn to the master Surface
    //     let output = self.surface.get_current_texture()?;
    //     let view = output
    //         .texture
    //         .create_view(&wgpu::TextureViewDescriptor::default());
    //     {
    //         let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //             label: None,
    //             color_attachments: &[wgpu::RenderPassColorAttachment {
    //                 view: &view,
    //                 resolve_target: None,
    //                 ops: wgpu::Operations {
    //                     load: clear_or_load,
    //                     store: true,
    //                 },
    //             }],
    //             depth_stencil_attachment: None,
    //         });
    //
    //         for obj in &self.objects {
    //             let my_model = &self.models[obj.model];
    //
    //             render_pass.set_pipeline(&self.render_pipelines[obj.pipeline]);
    //             render_pass.set_bind_group(0, &self.bind_groups[obj.bind_group], &[]);
    //             render_pass.set_vertex_buffer(0, my_model.vertex_buffer.slice(..));
    //             render_pass
    //                 .set_index_buffer(my_model.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    //             render_pass.draw_indexed(0..my_model.num_indices, 0, 0..1);
    //         }
    //     }
    // }

    // /// Renders any objects that request it, first to thier textures, and then the Panels.
    // pub fn render_loop(&mut self, force_surface_render: bool) -> Result<(), wgpu::SurfaceError> {
    //     (self.renderf)(&mut self, force_surface_render)
    // }


    pub fn api_loop(&mut self, redraw_request: bool, renderers: &mut Vec<TextureRenderer>, programs: &mut Vec<Box<dyn ProgramHook>>) -> Result<(), wgpu::SurfaceError> {

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // Only present the surface if it was drawn to
        let mut surface_output: Option<wgpu::SurfaceTexture> = None;

        // TEXTURE RENDERER RENDERPASSING
        // This is where panels
        let mut changing_statuses: Vec<&mut CallStatus> = vec![];
        for tex_rend in renderers {
            // deal with timing

            let mut i_should_render = match &mut tex_rend.drawf_status {
                CallStatus::Awaiting(timing) => match timing {
                    Timing::ASAP => true,
                    Timing::Framerate {
                        last_rendered_at,
                        desired_framerate,
                    } => {
                        if last_rendered_at.elapsed() >= fps_to_dur(*desired_framerate) {
                            true
                        } else {
                            false
                        }
                    }
                    Timing::SpecificTime { last_rendered_at, desired_wait_time } => {
                        if last_rendered_at.elapsed() >= *desired_wait_time {
                            true
                        } else {
                            false
                        }
                    }

                    Timing::Never => false
                },
                CallStatus::Inactive => {false}
                CallStatus::JustCalled(_) => {false}
            };

            // check force render
            if redraw_request {
                match tex_rend.texture {
                    TextureIndex::Surface => {
                        i_should_render = true;
                    }
                    _ => {}
                }
            }

            // render if i should
            if i_should_render {
                // call this texture's ProgramHook rendercall, get the new status
                programs[tex_rend.program_id.unwrap().clone()].render(tex_rend, self, &mut encoder);
                tex_rend.drawf_status = CallStatus::JustCalled(Instant::now());

                // check if texture was the Surface
                match tex_rend.texture {
                    TextureIndex::Surface => {
                        surface_output = Some(self.surface.get_current_texture()?);
                    }
                    TextureIndex::Index(_) => {}
                }

                // prepare to change the draw status
                changing_statuses.push(tex_rend.mut_draw_status());
            }
        }

        // render surface elements that are ready to render
        // self.render_surface(false, wgpu::LoadOp::Load)?;

        // submit commands for all renderpasses
        self.queue.submit(std::iter::once(encoder.finish()));
        // change all renderer statuses accordingly
        let time_rendered = Instant::now();
        for stat in changing_statuses {
            *stat = CallStatus::JustCalled(time_rendered.clone())
        }

        // preset to the screen if Surface was drawn
        match surface_output {
            None => {}
            Some(output) => {
                output.present();
            }
        };

        Ok(())
    }

}

