extends AcceptDialog


func _ready() -> void:
	get_close_button().hide()
	var err := connect("confirmed", self, "_on_confirmed", [], CONNECT_DEFERRED)
	if err:
		var msg : String = "AZDARA_PopError connecting 'confirmed' signal failed with Godot Error Code: " + String(err)
		push_error(msg)


func _on_confirmed():
	get_tree().quit(-1)






