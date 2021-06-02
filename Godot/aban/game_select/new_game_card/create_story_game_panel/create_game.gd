extends Panel


func _ready() -> void:
	var err := connect("resized", self, "check_orientation")
	if err :
		push_error("Create Game failed to connect 'resized' signal to himself.")
	check_orientation()


var orientation : int = HORIZONTAL

func check_orientation() -> void:
	var ratio : float = rect_size.x / rect_size.y
	if ratio < 1.4:
		if orientation != VERTICAL:
			orientation = VERTICAL
			construct_vertical()
	else:
		if orientation != HORIZONTAL:
			orientation = HORIZONTAL
			construct_horizontal()


func construct_vertical() -> void:
	pass


func construct_horizontal() -> void:
	pass













