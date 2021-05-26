extends Label


var time : float = 5
var mm : float = 1 # modulate_modifier


func _process(delta : float) -> void:
	if time < 0:
		queue_free()
	elif time < mm:
		modulate.a = (time / mm)
	time -= delta
