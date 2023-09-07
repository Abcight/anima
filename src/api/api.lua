-------------------------------------------------------------
--- Metatables! These are the types supported by the API. ---
-------------------------------------------------------------

prototypes = {}

prototypes.vec2 = {
	x = 0,
	y = 0,
	new = function(x, y)
		o = {}
		o.x = x
		o.y = y
		setmetatable(o, prototypes.vec2)
		o.__index = prototypes.vec2
		return o
	end,
	lerp = function(__self, other, t)
		return prototypes.vec2.new(
			lerp(__self.x, other.x, t),
			lerp(__self.y, other.y, t)
		)
	end
}

prototypes.vec3 = {
	x = 0,
	y = 0,
	z = 0,
	new = function(x, y, z)
		o = {}
		o.x = x
		o.y = y
		o.z = z
		setmetatable(o, prototypes.vec3)
		o.__index = prototypes.vec3
		return o
	end,
	lerp = function(__self, other, t)
		return prototypes.vec3.new(
			lerp(__self.x, other.x, t),
			lerp(__self.y, other.y, t),
			lerp(__self.z, other.z, t)
		)
	end
}

prototypes.color = {
	r = 0,
	g = 0,
	b = 0,
	new = function(r, g, b)
		o = {}
		o.r = r
		o.g = g
		o.b = b
		setmetatable(o, prototypes.color)
		o.__index = prototypes.color
		return o
	end,
	lerp = function(__self, other, t)
		return prototypes.color.new(
			lerp(__self.r, other.r, t),
			lerp(__self.g, other.g, t),
			lerp(__self.b, other.b, t)
		)
	end
}

-------------------------------------------------------------
---------- Shorthands for creating various objects ----------
-------------------------------------------------------------

function vec2(x, y)
	return prototypes.vec2.new(x, y)
end

function vec3(x, y, z)
	return prototypes.vec3.new(x, y, z)
end

function rgb(r, g, b)
	return prototypes.color.new(r, g, b)
end

-------------------------------------------------------------
--------------- Shorthand convenience methods ---------------
-------------------------------------------------------------

---Tries interpolating between a and b over t, if the underlying
---implementation exists.
---@param a any
---@param b any
---@param t number
---@return any
function lerp(a, b, t)
	if type(a) == "number" then
		return (1.0 - t) * a + t * b
	end

	return a.lerp(b, t)
end

---Draws a line from (x1, y1) to (x2, y2).
---@param x1 number
---@param y1 number
---@param x2 number
---@param y2 number
---@param color color
function line(x1, y1, x2, y2, color)
end