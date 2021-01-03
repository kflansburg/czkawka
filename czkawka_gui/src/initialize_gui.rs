use crate::connect_button_delete::{basic_remove, empty_folder_remover, tree_remove};
use crate::create_tree_view::*;
use crate::double_click_opening::*;
use crate::gui_data::*;
use crate::help_functions::*;
use directories_next::ProjectDirs;
use gtk::prelude::*;
use gtk::{SelectionMode, TreeView};
use image::imageops::FilterType;
use image::GenericImageView;
use std::cmp::Ordering;
use std::fs;
use std::path::Path;

pub fn initialize_gui(gui_data: &GuiData) {
    //// Setup default look(duplicate finder)
    {
        let buttons_search = gui_data.buttons_search.clone();
        let buttons_save = gui_data.buttons_save.clone();
        let buttons_delete = gui_data.buttons_delete.clone();
        let buttons_select = gui_data.buttons_select.clone();
        let buttons_symlink = gui_data.buttons_symlink.clone();
        let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
        let scrolled_window_main_empty_folder_finder = gui_data.scrolled_window_main_empty_folder_finder.clone();
        let scrolled_window_main_empty_files_finder = gui_data.scrolled_window_main_empty_files_finder.clone();
        let scrolled_window_main_temporary_files_finder = gui_data.scrolled_window_main_temporary_files_finder.clone();
        let scrolled_window_big_files_finder = gui_data.scrolled_window_big_files_finder.clone();
        let scrolled_window_similar_images_finder = gui_data.scrolled_window_similar_images_finder.clone();
        let scrolled_window_same_music_finder = gui_data.scrolled_window_same_music_finder.clone();
        let scrolled_window_invalid_symlinks = gui_data.scrolled_window_invalid_symlinks.clone();
        let scrolled_window_zeroed_files_finder = gui_data.scrolled_window_zeroed_files_finder.clone();
        let scrolled_window_included_directories = gui_data.scrolled_window_included_directories.clone();
        let scrolled_window_excluded_directories = gui_data.scrolled_window_excluded_directories.clone();
        let image_preview_similar_images = gui_data.image_preview_similar_images.clone();
        let check_button_settings_show_preview_similar_images = gui_data.check_button_settings_show_preview_similar_images.clone();
        let text_view_errors = gui_data.text_view_errors.clone();

        // Disable and show buttons
        buttons_search.show();
        buttons_save.hide();
        buttons_delete.hide();
        buttons_select.hide();
        buttons_symlink.hide();

        // Set Main Scrolled Window Treeviews
        {
            // Duplicate Files
            {
                let col_types: [glib::types::Type; 6] = [
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::U64,
                    glib::types::Type::String,
                    glib::types::Type::String,
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);
                tree_view.get_selection().set_select_function(Some(Box::new(select_function_duplicates)));

                create_tree_view_duplicates(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_duplicates);

                tree_view.connect_button_release_event(move |_tree_view, _e| {
                    // println!("{}", e.get_button());
                    gtk::Inhibit(false)
                });
                scrolled_window_duplicate_finder.add(&tree_view);
                scrolled_window_duplicate_finder.show_all();

                let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
                let gui_data = gui_data.clone();
                tree_view.connect_key_release_event(move |_tree_view, e| {
                    if let Some(button_number) = e.get_keycode() {
                        // Handle delete button
                        if button_number == 119 {
                            tree_remove(&scrolled_window_duplicate_finder, ColumnsDuplicates::Name as i32, ColumnsDuplicates::Path as i32, ColumnsDuplicates::Color as i32, &gui_data);
                        }
                    }
                    gtk::Inhibit(false)
                });
            }
            // Empty Folders
            {
                let col_types: [glib::types::Type; 3] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_empty_folders(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_empty_folders);

                scrolled_window_main_empty_folder_finder.add(&tree_view);
                scrolled_window_main_empty_folder_finder.show_all();

                let scrolled_window_main_empty_folder_finder = gui_data.scrolled_window_main_empty_folder_finder.clone();
                let gui_data = gui_data.clone();
                tree_view.connect_key_release_event(move |_tree_view, e| {
                    if let Some(button_number) = e.get_keycode() {
                        // Handle delete button
                        if button_number == 119 {
                            empty_folder_remover(&scrolled_window_main_empty_folder_finder, ColumnsEmptyFolders::Name as i32, ColumnsEmptyFolders::Path as i32, &gui_data);
                        }
                    }
                    gtk::Inhibit(false)
                });
            }
            // Empty Files
            {
                let col_types: [glib::types::Type; 3] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_empty_files(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_empty_files);

                scrolled_window_main_empty_files_finder.add(&tree_view);
                scrolled_window_main_empty_files_finder.show_all();

                let scrolled_window_main_empty_files_finder = gui_data.scrolled_window_main_empty_files_finder.clone();
                let gui_data = gui_data.clone();
                tree_view.connect_key_release_event(move |_tree_view, e| {
                    if let Some(button_number) = e.get_keycode() {
                        // Handle delete button
                        if button_number == 119 {
                            basic_remove(&scrolled_window_main_empty_files_finder, ColumnsEmptyFiles::Name as i32, ColumnsEmptyFiles::Path as i32, &gui_data);
                        }
                    }
                    gtk::Inhibit(false)
                });
            }
            // Temporary Files
            {
                let col_types: [glib::types::Type; 3] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_temporary_files(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_temporary_files);

                scrolled_window_main_temporary_files_finder.add(&tree_view);
                scrolled_window_main_temporary_files_finder.show_all();

                let scrolled_window_main_temporary_files_finder = gui_data.scrolled_window_main_temporary_files_finder.clone();
                let gui_data = gui_data.clone();
                tree_view.connect_key_release_event(move |_tree_view, e| {
                    if let Some(button_number) = e.get_keycode() {
                        // Handle delete button
                        if button_number == 119 {
                            basic_remove(&scrolled_window_main_temporary_files_finder, ColumnsTemporaryFiles::Name as i32, ColumnsTemporaryFiles::Path as i32, &gui_data);
                        }
                    }
                    gtk::Inhibit(false)
                });
            }
            // Big Files
            {
                let col_types: [glib::types::Type; 4] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_big_files(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_big_files);

                scrolled_window_big_files_finder.add(&tree_view);
                scrolled_window_big_files_finder.show_all();

                let scrolled_window_big_files_finder = gui_data.scrolled_window_big_files_finder.clone();
                let gui_data = gui_data.clone();
                tree_view.connect_key_release_event(move |_tree_view, e| {
                    if let Some(button_number) = e.get_keycode() {
                        // Handle delete button
                        if button_number == 119 {
                            basic_remove(&scrolled_window_big_files_finder, ColumnsBigFiles::Name as i32, ColumnsBigFiles::Path as i32, &gui_data);
                        }
                    }
                    gtk::Inhibit(false)
                });
            }
            // Similar Images
            {
                let image_preview_similar_images_clone = image_preview_similar_images.clone();
                image_preview_similar_images.hide();

                let col_types: [glib::types::Type; 10] = [
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::U64,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::U64,
                    glib::types::Type::String,
                    glib::types::Type::String,
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);
                tree_view.get_selection().set_select_function(Some(Box::new(select_function_similar_images)));

                create_tree_view_similar_images(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_similar_images);
                tree_view.connect_button_release_event(move |tree_view, _event| {
                    let (selected_rows, tree_model) = tree_view.get_selection().get_selected_rows();

                    let mut created_image = false;

                    if !selected_rows.is_empty() && check_button_settings_show_preview_similar_images.get_active() {
                        let tree_path = selected_rows[0].clone();
                        if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
                            // TODO labels on {} are in testing stage, so we just ignore for now this warning until found better idea how to fix this
                            #[allow(clippy::never_loop)]
                            'dir: loop {
                                let cache_dir = proj_dirs.cache_dir();
                                if cache_dir.exists() {
                                    if !cache_dir.is_dir() {
                                        add_text_to_text_view(&text_view_errors, format!("Path {} doesn't point at folder, which is needed by image preview", cache_dir.display()).as_str());
                                        break 'dir;
                                    }
                                } else if fs::create_dir_all(cache_dir).is_err() {
                                    add_text_to_text_view(&text_view_errors, format!("Failed to create dir {} needed by image preview", cache_dir.display()).as_str());
                                    break 'dir;
                                }
                                let path = tree_model.get_value(&tree_model.get_iter(&tree_path).unwrap(), ColumnsSimilarImages::Path as i32).get::<String>().unwrap().unwrap();
                                let name = tree_model.get_value(&tree_model.get_iter(&tree_path).unwrap(), ColumnsSimilarImages::Name as i32).get::<String>().unwrap().unwrap();

                                let file_name = format!("{}/{}", path, name);
                                let file_name = file_name.as_str();

                                if let Some(extension) = Path::new(file_name).extension() {
                                    let img = match image::open(&file_name) {
                                        Ok(t) => t,
                                        Err(_) => {
                                            add_text_to_text_view(&text_view_errors, format!("Failed to open temporary image file {}", file_name).as_str());
                                            break 'dir;
                                        }
                                    };
                                    if img.width() == 0 || img.height() == 0 {
                                        add_text_to_text_view(&text_view_errors, format!("Cannot create preview of image {}, with 0 width or height", file_name).as_str());
                                        break 'dir;
                                    }
                                    let ratio = img.width() / img.height();
                                    let requested_dimensions = (400, 400);
                                    let mut new_size;
                                    match ratio.cmp(&(requested_dimensions.0 / requested_dimensions.1)) {
                                        Ordering::Greater => {
                                            new_size = (requested_dimensions.0, (img.height() * requested_dimensions.0) / img.width());
                                            new_size = (std::cmp::max(new_size.0, 1), std::cmp::max(new_size.1, 1));
                                        }
                                        Ordering::Less => {
                                            new_size = ((img.width() * requested_dimensions.1) / img.height(), requested_dimensions.1);
                                            new_size = (std::cmp::max(new_size.0, 1), std::cmp::max(new_size.1, 1));
                                        }
                                        Ordering::Equal => {
                                            new_size = requested_dimensions;
                                            new_size = (std::cmp::max(new_size.0, 1), std::cmp::max(new_size.1, 1));
                                        }
                                    }
                                    let img = img.resize(new_size.0, new_size.1, FilterType::Triangle);
                                    let file_dir = cache_dir.join(format!("cached_file.{}", extension.to_string_lossy()));
                                    if img.save(&file_dir).is_err() {
                                        add_text_to_text_view(&text_view_errors, format!("Failed to save temporary image file to {}", file_dir.display()).as_str());
                                        break 'dir;
                                    }
                                    let string_dir = file_dir.to_string_lossy().to_string();
                                    image_preview_similar_images.set_from_file(string_dir);
                                    created_image = true;
                                }
                                break 'dir;
                            }
                        }
                    }
                    if created_image {
                        image_preview_similar_images.show();
                    } else {
                        image_preview_similar_images.hide();
                    }

                    gtk::Inhibit(false)
                });

                scrolled_window_similar_images_finder.add(&tree_view);
                scrolled_window_similar_images_finder.show_all();

                let scrolled_window_similar_images_finder = gui_data.scrolled_window_similar_images_finder.clone();
                let gui_data = gui_data.clone();
                tree_view.connect_key_release_event(move |_tree_view, e| {
                    if let Some(button_number) = e.get_keycode() {
                        // Handle delete button
                        if button_number == 119 {
                            tree_remove(
                                &scrolled_window_similar_images_finder,
                                ColumnsSimilarImages::Name as i32,
                                ColumnsSimilarImages::Path as i32,
                                ColumnsSimilarImages::Color as i32,
                                &gui_data,
                            );
                            image_preview_similar_images_clone.hide();
                        }
                    }
                    gtk::Inhibit(false)
                });
            }
            // Zeroed Files
            {
                let col_types: [glib::types::Type; 5] = [glib::types::Type::String, glib::types::Type::U64, glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_zeroed_files(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_zeroed_files);

                scrolled_window_zeroed_files_finder.add(&tree_view);
                scrolled_window_zeroed_files_finder.show_all();

                let scrolled_window_zeroed_files_finder = gui_data.scrolled_window_zeroed_files_finder.clone();
                let gui_data = gui_data.clone();
                tree_view.connect_key_release_event(move |_tree_view, e| {
                    if let Some(button_number) = e.get_keycode() {
                        // Handle delete button
                        if button_number == 119 {
                            basic_remove(&scrolled_window_zeroed_files_finder, ColumnsZeroedFiles::Name as i32, ColumnsZeroedFiles::Path as i32, &gui_data);
                        }
                    }
                    gtk::Inhibit(false)
                });
            }
            // Same Music
            {
                let col_types: [glib::types::Type; 13] = [
                    glib::types::Type::String,
                    glib::types::Type::U64,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::String,
                    glib::types::Type::U64,
                    glib::types::Type::String,
                    glib::types::Type::String,
                ];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_same_music(&mut tree_view);
                tree_view.get_selection().set_select_function(Some(Box::new(select_function_same_music)));

                tree_view.connect_button_press_event(opening_double_click_function_same_music);

                scrolled_window_same_music_finder.add(&tree_view);
                scrolled_window_same_music_finder.show_all();

                let scrolled_window_same_music_finder = gui_data.scrolled_window_same_music_finder.clone();
                let gui_data = gui_data.clone();
                tree_view.connect_key_release_event(move |_tree_view, e| {
                    if let Some(button_number) = e.get_keycode() {
                        // Handle delete button
                        if button_number == 119 {
                            tree_remove(&scrolled_window_same_music_finder, ColumnsSameMusic::Name as i32, ColumnsSameMusic::Path as i32, ColumnsSameMusic::Color as i32, &gui_data);
                        }
                    }
                    gtk::Inhibit(false)
                });
            }
            // Invalid Symlinks
            {
                let col_types: [glib::types::Type; 5] = [glib::types::Type::String, glib::types::Type::String, glib::types::Type::String, glib::types::Type::String, glib::types::Type::String];
                let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

                let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

                tree_view.get_selection().set_mode(SelectionMode::Multiple);

                create_tree_view_invalid_symlinks(&mut tree_view);

                tree_view.connect_button_press_event(opening_double_click_function_invalid_symlinks);

                scrolled_window_invalid_symlinks.add(&tree_view);
                scrolled_window_invalid_symlinks.show_all();

                let scrolled_window_invalid_symlinks = gui_data.scrolled_window_invalid_symlinks.clone();
                let gui_data = gui_data.clone();
                tree_view.connect_key_release_event(move |_tree_view, e| {
                    if let Some(button_number) = e.get_keycode() {
                        // Handle delete button
                        if button_number == 119 {
                            basic_remove(&scrolled_window_invalid_symlinks, ColumnsInvalidSymlinks::Name as i32, ColumnsInvalidSymlinks::Path as i32, &gui_data);
                        }
                    }
                    gtk::Inhibit(false)
                });
            }
        }

        // Set Included Directory
        {
            let col_types: [glib::types::Type; 1] = [glib::types::Type::String];
            let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

            let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

            tree_view.get_selection().set_mode(SelectionMode::Multiple);

            create_tree_view_directories(&mut tree_view);

            scrolled_window_included_directories.add(&tree_view);
            scrolled_window_included_directories.show_all();

            tree_view.connect_key_release_event(move |_tree_view, e| {
                if let Some(button_number) = e.get_keycode() {
                    // Handle delete button
                    if button_number == 119 {
                        let tree_view = get_tree_view(&scrolled_window_included_directories);
                        let list_store = get_list_store(&scrolled_window_included_directories);
                        let selection = tree_view.get_selection();

                        let (vec_tree_path, _tree_model) = selection.get_selected_rows();

                        for tree_path in vec_tree_path.iter().rev() {
                            list_store.remove(&list_store.get_iter(tree_path).unwrap());
                        }
                    }
                }
                gtk::Inhibit(false)
            });
        }
        // Set Excluded Directory
        {
            let col_types: [glib::types::Type; 1] = [glib::types::Type::String];
            let list_store: gtk::ListStore = gtk::ListStore::new(&col_types);

            let mut tree_view: gtk::TreeView = TreeView::with_model(&list_store);

            tree_view.get_selection().set_mode(SelectionMode::Multiple);

            create_tree_view_directories(&mut tree_view);

            scrolled_window_excluded_directories.add(&tree_view);
            scrolled_window_excluded_directories.show_all();

            tree_view.connect_key_release_event(move |_tree_view, e| {
                if let Some(button_number) = e.get_keycode() {
                    // Handle delete button
                    if button_number == 119 {
                        let tree_view = get_tree_view(&scrolled_window_excluded_directories);
                        let list_store = get_list_store(&scrolled_window_excluded_directories);
                        let selection = tree_view.get_selection();

                        let (vec_tree_path, _tree_model) = selection.get_selected_rows();

                        for tree_path in vec_tree_path.iter().rev() {
                            list_store.remove(&list_store.get_iter(tree_path).unwrap());
                        }
                    }
                }
                gtk::Inhibit(false)
            });
        }
    }
}