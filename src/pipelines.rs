use crate::wgpustate::State;
use std::collections::HashMap;
use std::fmt::{Display, Debug};
use crate::bytemuck::__core::fmt::Formatter;
use std::num::NonZeroU32;
use crate::wgpu::{BindGroupLayoutEntry, BindingType, TextureViewDimension};

/// Pipeline Styles will be the construction of Pipelines
/// each one having specific BindGroupLayouts that will have
/// a corresponding struct, such as RenderObject is to the default BindGroupLayout

// pub enum BindType {
//     Texture,
//     StorageTexture,
//     Sampler,
//     Buffer{structname: &str, structbody: &str, bindtype: wgpu::BindingType},
// }

// pub struct Bind {
//     btype: BindType,
//     binding: wgpu::BindingResource
// }


/// A copy of `wgpu::BindingType` but with added information.
pub enum BindSlot {
    /// A buffer binding.
    Buffer {

        /// The name of the struct which this data uses.
        struc_ty: &str,

        ty: wgpu::BufferBindingType,

        has_dynamic_offset: bool,

        min_binding_size: Option<wgpu::BufferSize>,
    },

    Sampler {
        filtering: bool,
        comparison: bool,
    },

    Texture {
        sample_type: wgpu::TextureSampleType,
        view_dimension: wgpu::TextureViewDimension,
        multisampled: bool,
    },

    StorageTexture {
        access: wgpu::StorageTextureAccess,
        format: wgpu::TextureFormat,
        view_dimension: wgpu::TextureViewDimension,
    },
}

/// A pipeline with a defined BindGroupLayout and shader program.
pub struct Pipeline {
    layout_id: usize,
    pipeline: usize,
    layout: HashMap<&str, (u32,wgpu::BindGroupLayoutEntry)>
}

pub struct PipelineConstructor {
    name: &str,
    struct_head: &str,
    header: String,
    src: String,
    /// * (Name, BindGroupLayoutEntry, Source)
    layout: Vec<(&str,wgpu::BindGroupLayoutEntry,String)>
}
impl PipelineConstructor {

    /// Struct definition source code for the shader module.
    pub fn struct_header(self, body: &str) -> Self {
        self.struct_head = body;
        self
    }

    pub fn with_binding(self, name: &str, visibility: wgpu::ShaderStages, bindtype: BindSlot, count: Option<NonZeroU32>) -> Self {

        let (bty,srcln) = match bindtype {
            BindSlot::Buffer { struc_ty, ty, has_dynamic_offset, min_binding_size } => (
                wgpu::BindingType::Buffer {
                        ty,
                        has_dynamic_offset,
                        min_binding_size
                    },
                format!("var<uniform> {}: {};\n",name,struc_ty)
            ),
            BindSlot::Sampler { filtering, comparison } => (
                wgpu::BindingType::Sampler {
                    filtering,
                    comparison
                },
                format!("var {}: sampler;\n",name)
            ),
            BindSlot::Texture { sample_type, view_dimension, multisampled } => {

                let dn = match view_dimension {
                    TextureViewDimension::D1 => "1d",
                    TextureViewDimension::D2 => "2d",
                    TextureViewDimension::D2Array => "2d_array",
                    TextureViewDimension::Cube => "cube",
                    TextureViewDimension::CubeArray => "cube_array",
                    TextureViewDimension::D3 => "3d"
                };

                (wgpu::BindingType::Texture {
                    sample_type,
                    view_dimension,
                    multisampled
                },
                 format!("var {}: texture_{};\n",name,dn)
                )
            }
            BindSlot::StorageTexture { access, format, view_dimension } => {
                unimplemented!()
            }
        };

        let this_bind_id = self.layout.len().clone();
        // TODO: add support for multiple groups   vvv
        self.header.push_str(format!("[[group(0), binding({})]]\n", this_bind_id).as_str());
        self.header.push_str(srcln.as_str());
        self.layout.push(
            (name, this_bind_id.clone(),
             wgpu::BindGroupLayoutEntry{
                 binding: this_bind_id,
                 visibility,
                 ty: bty,
                 count
             })
        );

        Self
    }

    pub fn create(self, state: &mut State) -> Pipeline {
        let pipeline_name = self.name.clone();
        let map = HashMap::new();
        let (names, entries): (Vec<&str>, Vec<wgpu::BindGroupLayoutEntry>) = self.layout.iter_mut().unzip();
        for i in 0..names.len() {
            map.insert(names[i].clone(), entries[i].clone())
        }

        let bglayout = state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: entries.as_slice(),
        });

        let bindgroup_layout_loc = state.bindgroup_layouts.len();
        state.bindgroup_layouts.push(bglayout);

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(self.src.as_str().into()),
        });


        let render_pipeline_layout =
            state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bglayout],
                push_constant_ranges: &[],
            });

    }
}

impl Pipeline {

    pub fn new(name: &str) -> PipelineConstructor {
        PipelineConstructor{
            name,
            struct_head: "",
            header: "".to_string(),
            src: String::from(""),
            layout: vec![]
        }
    }

    pub fn get_pipeline(&self, state: &State) -> &wgpu::RenderPipeline {
        &state.render_pipelines[self.pipeline]
    }

    /// Create a `BindGroup` by attaching `Bind`'s to the slot names specified as strings.
    /// Adds the `BindGroup` to the `State`, and returns it's ID.
    pub fn create_bindgroup(&self, state: &mut State, binds: Vec<(&str,wgpu::BindingResource)>) -> usize {
        let entries: Vec<wgpu::BindGroupEntry> = vec![];
        for (slot, bind) in binds {
            match self.layout.get(slot) {
                None => panic!("FATAL: Slot {} does not exist in Pipeline {}",slot,self.pipeline),
                Some((index,btype)) => {

                    // TODO: error handle on wrong binding types here

                    entries.push(wgpu::BindGroupEntry{
                        binding: index,
                        resource: bind
                    })

                }
            }
        }

        let bindgroup = state.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &state.bindgroup_layouts[self.layout_id],
            entries: entries.deref(),
        });

        state.bind_groups.push(bindgroup);
        state.bind_groups.len()-1
    }

}