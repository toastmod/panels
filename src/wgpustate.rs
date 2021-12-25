use crate::bindgroupreg::BindGroupReg;
use crate::modelbuffers::Model;
use crate::programhook::ProgramHook;
use crate::renderablestate::RenderableState;
use crate::renderobj::RenderObject;
use crate::resourcebytes::*;
use crate::texture;
use crate::texturerenderer::{TextureIndex, TextureRenderer, TextureViewQuery};
use crate::timing::{CallStatus, Timing};
use crate::transform2d::Transform2D;
use crate::util::fps_to_dur;
use image::GenericImageView;
use std::time::{Duration, Instant};
use wgpu::util::DeviceExt;
use wgpu::{SurfaceTexture, TextureView};
use winit::{event::*, window::*};
use crate::texture::Texture;
// use crate::pipelines::Pipeline;
use std::collections::HashMap;
use crate::pipelines::Pipeline;
use crate::rect::WorldPoint;

/// The render function for the WGPU `State`, defined by the user and called in the EventLoop
/// The `bool` parameter indicates a forced surface redraw request.
pub type StateRenderFunction = dyn Fn(&mut State, bool) -> Result<(), wgpu::SurfaceError>;

pub struct State {
    // pub renderf: Box<StateRenderFunction>,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipelines: Vec<wgpu::RenderPipeline>,
    /// A map for each pipeline.
    // pub pipeline_map: HashMap<&str,Pipeline>,
    pub models: Vec<Model>,
    pub uniform_buffers: Vec<wgpu::Buffer>,
    pub bindgroup_layouts: Vec<wgpu::BindGroupLayout>,
    pub bind_groups: Vec<wgpu::BindGroup>,

    /// All textures stored in this state.
    pub textures: Vec<texture::Texture>,

    pub pipeline_map: HashMap<String, Pipeline>,

    pub model_map: HashMap<String, usize>,

    // /// A hashmap for labelling RenderPipelines
    // pub pipeline_map: HashMap<&str, usize>,
    //
    // /// A hashmap for labelling BindGroupLayouts
    // pub bglayout_map: HashMap<&str, usize>,
    //
    // /// A hashmap for labelling BindGroups
    // pub bindgroup_map: HashMap<&str, usize>,

    // The renderers for all textures, including the main Surface.
    // pub texture_renderers: Vec<TextureRenderer>,

    // // Data for an object to render.
    // pub objects: Vec<RenderObject>,

    pub loop_fps: Option<f64>,

}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.enumerate_adapters(wgpu::Backends::all())
            .filter(|adapter| {
                // Check if this adapter supports our surface
                surface.get_preferred_format(&adapter).is_some()
            })
            .next()
            .unwrap();
            // .request_adapter(&wgpu::RequestAdapterOptions {
            //     power_preference: wgpu::PowerPreference::default(),
            //     force_fallback_adapter: false,
            //     compatible_surface: Some(&surface),
            // })
            // .await
            // .unwrap();

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

        let mut state = Self {
            // renderf,
            surface,
            device,
            queue,
            config,
            size,
            render_pipelines: vec![],
            // pipeline_map: HashMap::new(),
            models: vec![],
            uniform_buffers: vec![],
            bindgroup_layouts: vec![],
            bind_groups: vec![],
            textures: vec![],
            // texture_renderers: vec![],
            // objects: vec![],
            // pipeline_map: HashMap::new(),
            // bglayout_map: HashMap::new(),
            // bindgroup_map: HashMap::new(),
            pipeline_map: HashMap::new(),
            model_map: HashMap::new(),
            loop_fps: None
        };

        state.add_texture(texture::Texture::from_bytes(&state.device, &state.queue, diffuse_bytes).unwrap());

        // uniform buffer setup

        let pos_unif = WorldPoint::new(0.0,0.0,0.0);

        // models setup

        // TODO: create obj or from_slice model loader
        let vertex_buffer = state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(RECT_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(RECT_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = RECT_INDICES.len() as u32;

        state.models.push(Model {
            vertex_buffer,
            index_buffer,
            index_format: wgpu::IndexFormat::Uint16,
            offset_buffer: state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[pos_unif]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }),
            num_indices,
        });

        // state.uniform_buffers.push());

        state.add_pipeline("default::textured", |s| {

            // render pipeline setup
            let shader = s.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

            let panel_bind_group_layout = s.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

            let render_pipeline_layout =
                s.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&panel_bind_group_layout],
                    push_constant_ranges: &[],
                });
            (s.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                        format: s.config.format,
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
            }),
                panel_bind_group_layout
            )
        });

        state.create_bindgroup("default_textured", |s|{
            vec![
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&s.textures[0].view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&s.textures[0].sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: s.models[0].offset_buffer.as_entire_binding(),
                },
            ]
        });

        state

    }

    pub fn get_pipeline(&self, name: &str) -> &Pipeline {
        self.pipeline_map.get(&String::from(name)).unwrap()
    }

    /// Load a model with a data resource.
    pub fn load_model<V>(&mut self, name: &str, verticies: &[V], indicies: &[u16]) {
        unimplemented!()
    }

    /// Build a model by constructing Verticies and Indicies data for a buffer, and return the location of the model in the `models` vec in the `State` memory.
    /// V: Vertex type, must be compatible with pipeline.
    /// * Indicies should be in `u16`
    /// * Non-dynamic, non-rebuildable
    pub fn build_model<'a,V: bytemuck::Pod >(&mut self, name: &str, buildf: fn() -> (&'a [V],&'a [u16])) -> usize {

        let (va, ia) = buildf();

        let vb = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(va),
            usage: wgpu::BufferUsages::VERTEX
        });

        let ib = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(ia),
            usage: wgpu::BufferUsages::INDEX
        });


        let ob = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[WorldPoint::new(0.0,0.0,0.0)]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        });

        let model_id = self.models.len();
        self.models.push(Model{
            vertex_buffer: vb,
            index_buffer: ib,
            index_format: wgpu::IndexFormat::Uint16,
            offset_buffer: ob,
            num_indices: 0
        });

        self.model_map.insert(String::from(name), model_id);

        model_id
    }



    pub fn create_bindgroup(&mut self, pipeline: &str, buildf: fn(&State) -> Vec<wgpu::BindGroupEntry>) -> usize {
        let entries = buildf(&self);
        let p = &self.bindgroup_layouts[self.pipeline_map.get(&pipeline.to_string()).unwrap().bindgrouplayout];
        let bg = self.device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: None,
            layout: p,
            entries: entries.as_slice()
        });
        let bgid = self.bind_groups.len();
        self.bind_groups.push(bg);
        bgid
    }

    // TODO: make separate add functions for BindGroupLayouts, BindGroups, etc.
    /// Load a `wgpu::RenderPipeline` and `wgpu::BindGroupLayout` into `State` memory.
    pub fn add_pipeline(&mut self, name: &str, buildf: fn(&State) -> (wgpu::RenderPipeline, wgpu::BindGroupLayout)) {

        let (p,bgl) = buildf(&self);

        let pid = self.render_pipelines.len();
        self.render_pipelines.push(p);

        let bglid = self.bindgroup_layouts.len();
        self.bindgroup_layouts.push(bgl);

        self.pipeline_map.insert(String::from(name), Pipeline{
            pipeline: pid,
            bindgrouplayout: bglid
        });

    }

    pub fn create_renderobj(&self, pipeline: &str, model: usize, bind_group: usize) -> RenderObject {
        let p = self.pipeline_map.get(&pipeline.to_string()).unwrap();
        let m = &self.models[model];

        RenderObject{
            position: WorldPoint::new(0.0,0.0,0.0),
            pipeline: p.pipeline,
            bind_group,
            model,
            uniforms: vec![]
        }

    }

    /// Load a `Texture` into `State` memory.
    pub fn add_texture(&mut self, tex: Texture) -> usize {
        // self.bindgroup_layouts[0].
        let o = self.textures.len();
        self.textures.push(tex);
        o
    }

    // pub fn add_pipeline(&mut self, name: &str, desc: &wgpu::RenderPipelineDescriptor){
    //     let o = self.render_pipelines.len();
    //     self.render_pipelines.push(&self.device.create_render_pipeline(desc));
    //     self.pipeline_map.insert(name,o);
    // }
    //
    // pub fn add_bindgroup(&mut self, name: &str, desc: &wgpu::BindGroupDescriptor) {
    //     let o = self.bind_groups.len();
    //     self.bind_groups.push(&self.device.create_bind_group(desc));
    //     self.bindgroup_map.insert(name, o);
    // }
    //
    // pub fn add_bindgrouplayout(&mut self, name: &str, desc: &wgpu::BindGroupLayoutDescriptor) {
    //     let o = self.bindgroup_layouts.len();
    //     self.bindgroup_layouts.push(&self.device.create_bind_group_layout(desc));
    //     self.bglayout_map.insert(name, o);
    // }

    /// Get the Surface framerate.
    pub fn get_fps(&self) -> Option<f64> {
        self.loop_fps
    }

    /// Set the Surface framerate.
    pub fn set_fps(&mut self, fps: Option<f64>) {
        self.loop_fps = fps;
        // self.adjust_fps();
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Adjusts the FPS according to the `Timing` of each renderer's updatef and drawf.
    pub fn adjust_fps(&mut self) {
        todo!()
    }

    // pub fn input(&mut self, event: &WindowEvent) {
    //     match event {
    //         WindowEvent::Resized(_) => {}
    //         WindowEvent::Moved(_) => {}
    //         WindowEvent::CloseRequested => {}
    //         WindowEvent::Destroyed => {}
    //         WindowEvent::DroppedFile(_) => {}
    //         WindowEvent::HoveredFile(_) => {}
    //         WindowEvent::HoveredFileCancelled => {}
    //         WindowEvent::ReceivedCharacter(_) => {}
    //         WindowEvent::Focused(_) => {}
    //         WindowEvent::KeyboardInput { .. } => {}
    //         WindowEvent::ModifiersChanged(_) => {}
    //         WindowEvent::CursorMoved { .. } => {}
    //         WindowEvent::CursorEntered { .. } => {}
    //         WindowEvent::CursorLeft { .. } => {}
    //         WindowEvent::MouseWheel { .. } => {}
    //         WindowEvent::MouseInput { .. } => {}
    //         WindowEvent::TouchpadPressure { .. } => {}
    //         WindowEvent::AxisMotion { .. } => {}
    //         WindowEvent::Touch(_) => {}
    //         WindowEvent::ScaleFactorChanged { .. } => {}
    //         WindowEvent::ThemeChanged(_) => {}
    //     }
    // }

    // pub fn update(
    //     &mut self,
    //     redraw_request: bool,
    //     renderers: &mut Vec<TextureRenderer>,
    //     programs: &mut Vec<Box<dyn ProgramHook>>,
    // ) -> Result<(), wgpu::SurfaceError> {
    //     let mut encoder = self
    //         .device
    //         .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    //     let mut surface_texture = self.surface.get_current_texture()?;
    //     let mut surface_view: wgpu::TextureView = surface_texture
    //         .texture
    //         .create_view(&wgpu::TextureViewDescriptor::default());
    //     let mut texture_views: Vec<wgpu::TextureView> = vec![];
    //
    //     // note: scope here for renderpass ownership
    //     {
    //         let mut tex_rend = &mut renderers[0];
    //
    //         if tex_rend.should_call_updatef() {
    //             programs[tex_rend.program_id.unwrap().clone()].update(tex_rend, self);
    //         }
    //
    //         if tex_rend.should_call_drawf(redraw_request) {
    //             // get the correct TextureView to this renderer's Texture
    //             let view_ref = match tex_rend.get_textureview(self)? {
    //                 TextureViewQuery::RequestSurfaceView => &surface_view,
    //                 TextureViewQuery::View(v) => {
    //                     let view_id = texture_views.len();
    //                     texture_views.push(v);
    //                     &texture_views[view_id]
    //                 }
    //             };
    //
    //             // create a RenderPass based on the TextureRenderer's preference
    //             let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //                 label: None,
    //                 color_attachments: &[wgpu::RenderPassColorAttachment {
    //                     view: view_ref,
    //                     resolve_target: None,
    //                     ops: wgpu::Operations {
    //                         load: tex_rend.clear_or_load.clone(),
    //                         store: true,
    //                     },
    //                 }],
    //                 depth_stencil_attachment: None,
    //             });
    //
    //             // mutate the render_pass according to the program
    //             programs[tex_rend.program_id.unwrap().clone()].render(
    //                 tex_rend,
    //                 self,
    //                 &mut render_pass,
    //             );
    //         }
    //     }
    //
    //     self.queue.submit(std::iter::once(encoder.finish()));
    //
    //     // TODO: manage if the surface should be presented or not
    //     surface_texture.present();
    //
    //     Ok(())
    // }
}
