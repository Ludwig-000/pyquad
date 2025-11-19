import pyquad


pyquad.activate_engine()


size =pyquad.Vec3(1,1,1)
pos = pyquad.Vec3(1,1,1)
rot = pyquad.Vec3(0,0,0)

myCube = pyquad.Cube(size,pos,rot)  #  <- the line that crashes the test.
pyquad.draw_all_objects()

while True: # regular code to run later
    pyquad.draw_rectangle(100,100,100,100,pyquad.Color.RED)
    pyquad.next_frame()