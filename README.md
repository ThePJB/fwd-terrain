# FWD Terrain Gen
* Have grayscale type
* Have RGB type (for normals)
* Designing a process, we can make it stay within certain confines
    * min height, max height
    * min variance, max variance
* volcanos -- especially where you get chains of them from a hotspot and a shiftings
* plates converge
* plates diverge
* random plane -- noise from random planes is a thing tho
* hydraulic erosion please
* gaussian LPF shit too


## Considerations
* Whether to actually store normals when they will need all recomputing? Or to just quick and dirty estimate at all times. normal_bilinear, normal_bicubic

## MVP
* volcano (ideally its like plus of cone at certain location)
* weathering LPF
* translate