#!/usr/bin/env python3

import struct
import sys

with open(sys.argv[1]) as f:
    lines = f.readlines()

faces = [line[2:] for line in lines if line.startswith("f ")]
vertices = [line[2:] for line in lines if line.startswith("v ")]
normals = [line[3:] for line in lines if line.startswith("vn ")]

vs = []
ns = []


def add(vertex):
    v, t, n = map(int, vertex.split('/'))
    vs.append(list(map(float, vertices[v - 1].strip().split())))
    ns.append(list(map(float, normals[n - 1].strip().split())))


for face in faces:
    verticies = face.strip().split()
    vc = len(verticies)
    if vc == 4:
        for vertex in verticies[0:3]:
            add(vertex)
        for vertex in [verticies[0], verticies[2], verticies[3]]:
            add(vertex)
    if vc == 3:
        for vertex in verticies[0:3]:
            add(vertex)
    print(vc)

with open("suzanne.cgi", "wb") as f:
    for v, n in zip(vs, ns):
        f.write(struct.pack("<dddddd", *(v + n)))

print(f"Saved {len(vs)} verticies")
