class_name Entity
extends Node


func _ready() -> void:
	$Visual.teleport()
	pass

func set_position(pos : Vector2) -> void:
	$Physics.position = pos
	$Visual.teleport()


func set_static() -> void:
	$Physics.set_mode(RigidBody2D.MODE_STATIC)


func get_visual() -> Node2D:
	return ($Visual as Node2D)
