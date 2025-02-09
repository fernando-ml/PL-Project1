import json
import math
import sys
from typing import List

class SolarSystemBody:
    def __init__(self, name: str, diameter: float = 0, circumference: float = 0):
        self.name = name
        self.diameter = diameter
        self.circumference = circumference

    def calculate_circumference(self) -> None:
        if self.diameter != 0 and self.circumference == 0:
            self.circumference = self.diameter * math.pi

    def calculate_diameter(self) -> None:
        if self.circumference != 0 and self.diameter == 0:
            self.diameter = self.circumference / math.pi

    def calculate_volume(self) -> float:
        radius = self.diameter / 2
        return (4/3) * math.pi * (radius ** 3)

class Moon(SolarSystemBody):
    def __str__(self) -> str:
        return (f"Moon: {self.name}\n"
                f"Diameter: {self.diameter:,.1f} km\n"
                f"Circumference: {self.circumference:,.1f} km"
                )

class Planet(SolarSystemBody):
    def __init__(self, name: str, distance_from_sun: float = 0, 
                 orbital_period: float = 0, diameter: float = 0, 
                 circumference: float = 0, moons: List[dict] = None):
        super().__init__(name, diameter, circumference)
        self.distance_from_sun = distance_from_sun
        self.orbital_period = orbital_period
        self.moons: List[Moon] = []
        if moons:
            self.add_moons(moons)

    def add_moons(self, moons_data: List[dict]) -> None:
        for moon_data in moons_data:
            moon = Moon(
                name=moon_data["Name"],
                diameter=moon_data.get("Diameter", 0),
                circumference=moon_data.get("Circumference", 0)
            )
            moon.calculate_circumference()
            moon.calculate_diameter()
            self.moons.append(moon)

    def calculate_orbital_period(self) -> None:
        if self.distance_from_sun != 0 and self.orbital_period == 0:
            self.orbital_period = math.sqrt(self.distance_from_sun ** 3)

    def calculate_distance(self) -> None:
        if self.orbital_period != 0 and self.distance_from_sun == 0:
            self.distance_from_sun = (self.orbital_period ** 2) ** (1/3)

    def __str__(self) -> str:
        output = [
            f"Planet: {self.name}",
            f"Distance from sun: {self.distance_from_sun:.2f} au",
            f"Orbital period: {self.orbital_period:.2f} yr",
            f"Diameter: {self.diameter:,.0f} km",
            f"Circumference: {self.circumference:,.0f} km"
        ]
        
        for moon in self.moons:
            output.append(str(moon))
        
        return "\n".join(output)


class Sun(SolarSystemBody):
    def __init__(self, name: str, diameter: float = 0, 
                 circumference: float = 0, planets: List[dict] = None):
        super().__init__(name, diameter, circumference)
        self.planets: List[Planet] = []
        if planets:
            self.add_planets(planets)

    def add_planets(self, planets_data: List[dict]) -> None:
        for planet_data in planets_data:
            planet = Planet(
                name=planet_data["Name"],
                distance_from_sun=planet_data.get("DistanceFromSun", 0),
                orbital_period=planet_data.get("OrbitalPeriod", 0),
                diameter=planet_data.get("Diameter", 0),
                circumference=planet_data.get("Circumference", 0),
                moons=planet_data.get("Moons", [])
            )
            planet.calculate_circumference()
            planet.calculate_diameter()
            planet.calculate_orbital_period()
            planet.calculate_distance()
            self.planets.append(planet)

    def total_planets_volume(self) -> float:
        return sum(planet.calculate_volume() for planet in self.planets)

    def __str__(self) -> str:
        output = [
            f"Sun: {self.name}",
            f"Diameter: {self.diameter:,.0f} km",
            f"Circumference: {self.circumference:,.0f} km\n"
        ]
        
        for planet in self.planets:
            output.append(str(planet))
            output.append("")  # we added an empty line after each planet for readability
            
        sun_volume = self.calculate_volume()
        planets_volume = self.total_planets_volume()
        output.append(f"All the planets' volumes added together could fit in the Sun: {planets_volume < sun_volume}")
        
        return "\n".join(output)


def main():
    if len(sys.argv) != 2:
        print("How to run: python solar_system.py <json_file>")
        sys.exit(1)

    try:
        with open(sys.argv[1], 'r') as file:
            data = json.load(file)
    except FileNotFoundError:
        print(f"Error: Could not find file {sys.argv[1]}")
        sys.exit(1)
    except json.JSONDecodeError:
        print("Error: Invalid JSON format")
        sys.exit(1)

    # create solar system
    sun = Sun(
        name=data["Name"],
        diameter=data.get("Diameter", 0),
        circumference=data.get("Circumference", 0),
        planets=data.get("Planets", [])
    )
    sun.calculate_circumference()
    sun.calculate_diameter()
    print(sun)

if __name__ == "__main__":
    main()