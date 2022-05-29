# modl
`modl` is able to display 3D models!

#### TODO
Make an application that is able to display a 3D model. 
It must:
1. Be able to import .obj files
2. Utilize an ECS
3. Support diffuse textures
4. Support normal maps
5. Have configurable lighting
---

- [x] Write TODO
- Complete https://sotrh.github.io/learn-wgpu/
  - [x] 3d Camera
  - [x] Instancing
  - [x] Depth buffer
  - [x] model loadiiiiiiing
  - [x] Lighting
  - [ ] Normal mapping
  - [ ] Even better camera
- [ ] Make Renderer lib (with a bunch of state and stuff. Expose traits for uniforms and buffer descriptions)
- [ ] Multiple lights
- [ ] rotation and scale in instances
- [ ] Import Bevy ECS. 
  - [ ] Transform component
  - [ ] Model component (just inline the mesh and material data)
  - [ ] Light component
  - [ ] Pass a world to renderer for rendering.
  - [ ] Asset library (use references in Mesh/Material components)


Feature ideas:
Frametime logging
Shadows (via shadow mapping)
Skybox (via Cubemaps)
pbr rendering stolen from bevy?
screen effects (bloom, vingette, etc.)
imgui?
