The idea of this project is to create N-dimensional cubes, with parent-child hierarchies.

Starting with 3 dimensions I aim to create a fast system to aggregate totals of queries at any level. (I have many ideas of how to expand this for more than 3 dimensions but I'll get around to this later).

As a simple example consider these trees/heirarchies:

1:   TIME:
        2023:
             Q1:
             Q2:
             Q3:
             Q4:
        2024:
             Q1:
             Q2:
             Q3:
             Q4:        

2:   AREA:
        North:
        South:
        East:
        West:
        
3:   COST:
        Wages:  
        Utilities:
        Rent:

For each point in TIME, for a given AREA, there may be a COST

Given some base data it should be possible to agrregate a specific COST for an area and a time period:

e.g. starting with (a small sample set)
2023:Q1/North/Wages = 20212.3
2023:Q2/North/Wages = 20123.4 
2023:Q3/North/Wages = 22100.9
2023:Q4/North/Wages = 33033.3

We shoud be able to calculate that the money spent on Wages, in the North, in 2023 = 96469.9 (If I've summed correctly ; )

Such calculations should be possible for ANY depth of hierarchy (e.g 
   time periods split into individual months/weeks/days, 
   Areas devided by country, region, city, postcode, street 
   Costs split into a more granular state -- e.g. wages becoming base salary, pension contribution, insurance, tax etc.
)

Ideally i'd hope that that each ITEM (e.g TIME/AREA/COST) should be represented by a 32 bit integer. This should help with calculations for alllocating memory (and the size of each record should be nicely aligned for reading using a 64 bit processor).

So... for each triplet assigned we'd use 128bits (32 for TIME_ID, 32 for AREA_ID, 32 for COST_ID).... Then either 32 bits to represent a floating point value,   or a further integer to point to a secondary cube....

ANY cube in the system should workd like this.  Cubes should either contain a VALUE as the last 32 bits, or an ID into another cube.







