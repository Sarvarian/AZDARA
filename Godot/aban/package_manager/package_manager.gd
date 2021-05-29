extends Node


const packages_directories : Array = [
	"user://packages/",
	"res://packages/"
]

var packages : Dictionary = {}


func _ready() -> void:
	check_directories_for_packages()
	print(packages)

func  check_directories_for_packages() -> void:
	var err : int = 0
	var directory : Directory = Directory.new()
	
	for dir in packages_directories:
		
		# Create directory if not exist
		if not directory.dir_exists(dir):
			err = directory.make_dir_recursive(dir)
			if err:
				var msg : String = "Creating Packages Directory at " + dir + " Failed With Godot Error Code: " + String(err)
				Log.error(msg)
				push_error(msg)
		
		# Open directory
		err = directory.open(dir)
		if err:
			var msg : String = "Opening Directory '" + dir + "' Failed With Godot Error Code: " + String(err)
			Log.error(msg)
			push_error(msg)
		
		# Setup directory fo iteration
		err = directory.list_dir_begin(true, false)
		if err:
			var msg : String = "'" + dir + "'.list_dir_begin(true, false) Failed With Godot Error Code: " + String(err)
			Log.error(msg)
			push_error(msg)
		
		# Iterate and add subdirectories to packages
		var name := directory.get_next()
		while name != "":
			if directory.current_is_dir():
				packages[name] = dir + name
			name = directory.get_next()
		
		directory.list_dir_end()








