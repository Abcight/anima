-------------------------------------------------------------
--------- █████╗  ███╗   ██╗██╗███╗   ███╗ █████╗  ----------
--------- ██╔══██╗████╗  ██║██║████╗ ████║██╔══██╗ ----------
--------- ███████║██╔██╗ ██║██║██╔████╔██║███████║ ----------
--------- ██╔══██║██║╚██╗██║██║██║╚██╔╝██║██╔══██║ ----------
--------- ██║  ██║██║ ╚████║██║██║ ╚═╝ ██║██║  ██║ ----------
--------- ╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝╚═╝     ╚═╝╚═╝  ╚═╝ ----------
-------------------------------------------------------------

-------------------------------------------------------------
---             Use this file as a reference!             ---
---                 🌏︎ abcight.com/anima                  ---
---                ✉ contact@abcight.com                 ---
-------------------------------------------------------------

-------------------------------------------------------------
--- Metatables! These are the types supported by the API. ---
--- (These are purposefully hidden from the user! Scram!) ---
-------------------------------------------------------------

prototypes = {}

---@class vec2
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

---@class vec3
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

---@class color
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

---@class transition
prototypes.transition = {
	from = nil,
	to = nil,
	start = nil,
	duration = nil,
	fn = nil,
	end_counted = false
}

prototypes.transition.__index = prototypes.transition

function prototypes.transition:new(from, to, duration, fn)
	local o = {}
	setmetatable(o, prototypes.transition)

	o.start = LAST_AWAIT_END
	o.from = from
	o.to = to
	o.duration = duration
	o.fn = fn
	
	return o
end

--- Pauses until the transition is finished
function prototypes.transition:await()
	if self.end_counted then
		return
	end

	local local_time = math.max(0, TIME - self.start)
	local normalized_time = local_time / self.duration
	if normalized_time < 1.0 then
		__interrupt()
	else
		self.end_counted = true
		LAST_AWAIT_END = math.max(LAST_AWAIT_END, self.start + self.duration)
	end
end

--- Returns the current value of transition
function prototypes.transition:current()
	local fn = self.fn or function(x) return x end

	local local_time = math.max(0, TIME - self.start)
	local normalized_time = local_time / self.duration
	local fn_time = fn(normalized_time)

	if normalized_time < 1.0 then
		return lerp(self.from, self.to, fn_time)
	end

	return self.to
end

-------------------------------------------------------------
---------- Shorthands for creating various objects ----------
-------------------------------------------------------------

---Creates a two-dimensional vector (x, y).
---@param x number
---@param y number
---@return vec2
function vec2(x, y)
	return prototypes.vec2.new(x, y)
end

---Creates a three-dimensional vector (x, y, z).
---@param x number
---@param y number
---@param z number
---@return vec3
function vec3(x, y, z)
	return prototypes.vec3.new(x, y, z)
end

---Creates a color from rgb values.
---@param r number
---@param g number
---@param b number
---@return color
function rgb(r, g, b)
	return prototypes.color.new(r, g, b)
end

-------------------------------------------------------------
--------------- Shorthand convenience methods ---------------
-------------------------------------------------------------

---Tries interpolating between a and b over t, if the underlying
---implementation exists.
---@alias impl_lerp number | vec2 | vec3 | color
---@param a impl_lerp
---@param b impl_lerp
---@param t number
---@return impl_lerp
function lerp(a, b, t)
	if type(a) == "number" then
		return (1.0 - t) * a + t * b
	end

	return a.lerp(b, t)
end

-------------------------------------------------------------
--------------------- Timeline shenans! ---------------------
-------------------------------------------------------------

TIME = 0
LAST_AWAIT_END = 0
MODE_ACCUMULATOR = false

function __interrupt()
	if MODE_ACCUMULATOR then return end
	
	---@diagnostic disable-next-line: undefined-global
	__a_function_that_totally_doesnt_exist()
end

---Creates a transition between a and b over t, if the underlying
---implementation exists.
---@param from impl_lerp
---@param to impl_lerp
---@param duration number
---@param lerp_fn function<impl_lerp>
function transition(from, to, duration, lerp_fn)
	return prototypes.transition:new(
		from,
		to,
		duration,
		lerp_fn
	)
end

-------------------------------------------------------------
----------------------- Rust bindings! ----------------------
-------------------------------------------------------------

---Draws a line from (x1, y1) to (x2, y2).
---@param x1 number
---@param y1 number
---@param x2 number
---@param y2 number
---@param thickness number
---@param color color
function line(x1, y1, x2, y2, thickness, color)
end

---Draws a line from (x<sub>a</sub>, y<sub>a</sub>) to (x<sub>b</sub>, y<sub>b</sub>).
---@param a vec2
---@param b vec2
---@param thickness number
---@param color color
function line_v(a, b, thickness, color)
end

---Draws a circle at (x, y) with the specified radius and color.
---@param x number
---@param y number
---@param radius number
---@param color color
function circle(x, y, radius, color)
end

---Draws a circle at (x<sub>o</sub>, y<sub>o</sub>) with the specified radius and color.
---@param o vec2
---@param radius number
---@param color color
function circle_v(o, radius, color)
end