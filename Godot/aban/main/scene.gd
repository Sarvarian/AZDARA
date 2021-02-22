extends Node


func clear_scene():
	for c in get_children():
		(c as Node).queue_free()
		remove_child(c)
