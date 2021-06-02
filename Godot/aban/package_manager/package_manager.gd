extends Node


const packages_directories : Array = [
	"user://packages/",
	"res://packages/"
]


var packages : Dictionary = {}
var scenarios : Dictionary = {}
var directory : Directory = Directory.new()
var file : File = File.new()
var rust := preload("res://aban/lib/package_manager.gdns").new()


func _ready() -> void:
	check_directories_for_packages()
	check_packages_for_scenarios()


func check_directories_for_packages() -> void:
	packages.clear()
	for dir in packages_directories:
		var err : int = 0
		
		# Create directory if not exist
		if not directory.dir_exists(dir):
			err = directory.make_dir_recursive(dir)
			if err:
				var msg : String = "Creating Packages Directory at " + dir + " Failed With Godot Error Code: " + String(err)
				Log.error(msg)
				push_error(msg)
				continue
		
		# Open directory
		err = directory.open(dir)
		if err:
			var msg : String = "Opening Directory '" + dir + "' Failed With Godot Error Code: " + String(err)
			Log.error(msg)
			push_error(msg)
			continue
		
		# Setup directory fo iteration
		err = directory.list_dir_begin(true, false)
		if err:
			var msg : String = "'" + dir + "'.list_dir_begin(true, false) Failed With Godot Error Code: " + String(err)
			Log.error(msg)
			push_error(msg)
			continue
		
		# Iterate and add subdirectories to packages
		var name := directory.get_next()
		while name != "":
			if directory.current_is_dir():
				packages[name] = dir + name
			name = directory.get_next()
		
		directory.list_dir_end()


func check_packages_for_scenarios() -> void:
	scenarios.clear()
	var err : int
	var path : String
	for pack in packages:
		path = packages[pack] as String + "/scenario.json"
		err = file.open(path, File.READ)
		
		if err and err != ERR_FILE_NOT_FOUND:
			var msg := "Error on reading file '" + path + "' Godot Error Code: " + String(err)
			Log.error(msg)
			push_error(msg)
			continue
		
		var text := file.get_as_text()
		file.close()
		
		var json : JSONParseResult = JSON.parse(text)
		err = json.error # TODO json error handling
		if err:
			var parse_err := "Line: " + String(json.error_line) + " Error: " + json.error_string
			var msg := "Error on parsing json from file '" + path + "' Godot Error Code: " + String(err) + "\n" + parse_err
			Log.error(msg)
			push_error(msg)
			continue
		
		scenarios[pack] = json.result






















