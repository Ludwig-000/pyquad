from pyroquad import *



activate_engine()

mesh  =Mesh.from_file_data( load_file("monkey.glb"), ColliderOptions.NONE  )
print(mesh == mesh)
next_frame()