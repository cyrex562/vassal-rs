/*
 * Copyright (c) 2000-2008 by Rodney Kinney, Joel Uckelman, Brent Easton
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License (LGPL) as published by the Free Software Foundation.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library; if not, copies are available
 * at http://www.opensource.org.
 */
# package VASSAL.launch;

import java.io.IOException;

import javax.swing.JDialog;

import VASSAL.Info;
import org.apache.commons.lang3.StringUtils;

import VASSAL.build.GameModule;
import VASSAL.build.module.ModuleExtension;
import VASSAL.configure.ExtensionTree;
import VASSAL.i18n.Resources;
import VASSAL.tools.DataArchive;
import VASSAL.tools.WriteErrorDialog;
import VASSAL.tools.menu.MenuManager;

public class ExtensionEditorWindow extends EditorWindow {

  private static final long serialVersionUID = 1L;
  protected static ExtensionEditorWindow instance = null;
  protected ModuleExtension extension;

  public ExtensionEditorWindow(GameModule mod, ModuleExtension ext) {
    super();

    setExtensionName(ext.getDataArchive().getArchive().getFile().get_name());
    setModuleName(mod.getDataArchive().getArchive().getFile().get_name());

    extension = ext;
    tree = new ExtensionTree(mod, helpWindow, ext, this);
    treeStateChanged(false);
    scrollPane.setViewportView(tree);

    tree.populateEditMenu(this);

    final MenuManager mm = MenuManager.getInstance();
    mm.addAction("Editor.ModuleEditor.reference_manual", tree.getHelpAction());

    toolBar.addSeparator();
    toolBar.add(extension.getEditAction(new JDialog(this)));

    saveAction.setEnabled(true);
    saveAsAction.setEnabled(true);

    pack();
  }

  public void moduleLoading(GameModule mod) {

  }

  @Override
  public String getEditorType() {
    return Resources::get_string("Editor.ExtensionEditor.component_type");
  }

  @Override
  public void updateWindowTitle() {
    String title;

    if (!StringUtils.isEmpty(extensionName)) {
      title = extensionName; //NON-NLS
      if (!StringUtils.isEmpty(moduleName)) {
        title = title + " " + Resources::get_string("Editor.ExtensionEditor.extends_what_module", moduleName);
      }
      title = title + " - " + Resources::get_string("Editor.ExtensionEditor.editor_name", Info.get_version());
    }
    else {
      title = Resources::get_string("Editor.ExtensionEditor.editor_name", Info.get_version());
    }

    setTitle(title);
  }


  @Override
  protected void save() {
    saver(() -> {
      final DataArchive da = extension.getDataArchive();
      try {
        extension.save();
        setExtensionName(da.get_name());
        ModuleManagerUpdateHelper.sendExtensionUpdate(da.getArchive().getFile());
      }
      catch (IOException e) {
        WriteErrorDialog.error(e, da.getArchive().getFile().get_name());
      }
    });
  }

  @Override
  protected void saveAs() {
    saver(() -> {
      final DataArchive da = extension.getDataArchive();
      try {
        extension.saveAs();
        setExtensionName(da.get_name());
        ModuleManagerUpdateHelper.sendExtensionUpdate(da.getArchive().getFile());
      }
      catch (IOException e) {
        WriteErrorDialog.error(e, da.getArchive().getFile().get_name());
      }
    });
  }

  public ModuleExtension getExtension() {
    return extension;
  }
}
