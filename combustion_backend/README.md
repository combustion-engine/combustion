`combustion_backend`
--------------------

The combustion backend is the foundation for interacting with hardware and OS APIs, so the rest of the engine doesn't have to worry about the specifics of those.

While still in progress, I hope to provide a uniform API for interacting with OpenGL and Vulkan at the very least, with possible DirectX support in the far future.

### Current progress:

- [x] OpenGL backend
- [ ] Vulkan backend
- [ ] DirectX 11 backend
- [ ] DirectX 12 backend

### API progress:

- [x] Low level wrappers
- [ ] Command queue interface
- [ ] Abstract rendering pipeline
- [ ] Bindless rendering