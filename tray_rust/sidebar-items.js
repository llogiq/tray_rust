initSidebarItems({"mod":[["bxdf","Defines the BxDF interface implemented by BRDF/BTDFs to describe material properties. Also provides the BSDF type which composes various BRDF/BTDFs to describe materials"],["exec","The exec module provides an abstraction of the execution backends used to actually render the image"],["film","The film module provides color types and a render target that the image is written too."],["geometry","The geometry module defines the Geometry trait implemented by the various geometry in the ray tracer and provides some standard geometry for rendering"],["integrator","The integrator module defines the Integrator trait implemented by the various surface integrators used to render the scene with different integration methods, eg. path tracing, photon mapping etc."],["light","Defines the light interface implemented by all lights in tray_rust and the OcclusionTester which provides a convenient interface for doing shadow tests for lights"],["linalg","The linalg module provides some basic linear algebra functionality for transforming 3D geometry"],["material","Defines the trait implemented by all materials and exports various supported material types. Materials are used to define how BxDFs are composed to create the desired appearance"],["mc","Defines various Monte Carlo sampling functions for sampling points/directions on objects and computing the corresponding pdfs"],["partition","Provides a general partitioning function that implements C++'s std::partition"],["sampler","Provides the Sampler trait which is implemented by the various samplers to provide stratified, low-discrepancy, adaptive sampling methods and so on through a simple trait interface"],["scene","Defines the scene struct which contains the various objects defining the scene. This includes the geometry, instances of the geometry, the camera and so on."]]});