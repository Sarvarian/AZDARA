extends ViewportContainer


onready var character := $Viewport/Character
onready var camera := $Viewport/Camera


func set_world_2d(world_2d : World2D) -> void:
	$Viewport.world_2d = world_2d

