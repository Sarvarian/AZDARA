extends Camera2D


var target : Node2D


func _ready() -> void:
#	if not target:
#		queue_free()
#		get_parent().remove_child(self)
#		return
	smoothing_enabled = true
	current = true
	pass


func _process(_delta: float) -> void:
	global_position = target.global_position
	pass
