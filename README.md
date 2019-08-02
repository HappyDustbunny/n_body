# n_body
N-body simuation as an exercise in Rust using Nannou for graphics and Crossbeam for multi threading.  
  
\
This program simulates a collision of two star clusters (CL1 and CL2).    CL1 starts at rest at origo.

At the commandline the program needs the number of stars in each cluster, their radii and the initial position and velocity of CL2.

Ex: &nbsp; &nbsp; n_body &nbsp; &nbsp; #_of_stars_CL1 &nbsp;  &nbsp; #_of_stars_CL2 &nbsp; &nbsp; radiusCL1 &nbsp; radiusCL2 &nbsp; x y z &nbsp; vx vy vz

Ex: &nbsp; &nbsp; n_body 1500 100 3000.0 2000.0 6000.0 0.0 0.0 -1.0 0.0 0.0  

\
Read Theory.odt to get details of why we set G = 1 in Newton's gravity law.

<img src="/Cluster.gif?raw=true" align="left">  

\
Two clusters of 15000 stars collide.   

\
The simulation runs slower. This gif shows about 1:30 min runtime.
