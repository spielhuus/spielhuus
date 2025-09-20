+++
title = 'Voronoi'
description = 'drawing voronoi diagrams'
date = 2025-09-06T11:35:00+02:00
draft = true
tags = ['ca']
script = "voronoi/js/main.ts"
+++

Drawing Voronoi diagrams is very simle. 

1) Create random points on the screen
1) assign a color to the points
1) for each pixel search the closest point
1) assign the color of this point to this pixel

<figure>
<canvas width=1280 height=860 id="shader"></canvas>
</figure>


## Links:
- {{< link "wolfram" >}}.
- {{< github "ca_rules" >}}.

{{< bindgen path="js/voronoi/voronoi.js" >}}
