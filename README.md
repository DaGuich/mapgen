# Information

# Usage

## Import Data

## Generate one Tile

## Generate multiple tiles

## Run tile server

# Implementation Details

## Format of imported data

## Formulas used

### Retrieve Tile

For a latitude $\varphi$ and a longitude $\lambda$, both in radians, we can 
calculate the n_th tile in $x$ and $y$ direction for a given zoom level $z$.

$$
x = \lfloor {\varphi + \pi \over 2\pi} \cdot 2^{z} \rfloor
$$

$$
y = \lfloor (1 - {\ln ( \tan \varphi + {1 \over \cos \varphi}) \over \pi}) \cdot 2^{z-1} \rfloor
$$

### Get bounding box of tile
