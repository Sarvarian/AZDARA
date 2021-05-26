class_name MapBase
extends Viewport


func _init():
	own_world = true
	disable_3d = true
	usage = Viewport.USAGE_2D
	world = World.new()
	world_2d = World2D.new()
