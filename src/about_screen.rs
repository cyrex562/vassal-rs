
// package VASSAL.build.module.documentation;

// import VASSAL.Info;
// import VASSAL.build.AbstractConfigurable;
// import VASSAL.build.Buildable;
// import VASSAL.build.GameModule;
// import VASSAL.build.module.Documentation;
// import VASSAL.build.module.ModuleExtension;
// import VASSAL.i18n.Resources;
// import VASSAL.tools.imageop.ImageOp;
// import VASSAL.tools.imageop.Op;
// import VASSAL.tools.menu.MenuManager;
// import VASSAL.tools.swing.AboutWindow;
//
// import javax.swing.AbstractAction;
// import javax.swing.Action;
// import java.awt.Image;
// import java.awt.event.ActionEvent;
// import java.io.File;
// import java.util.Collection;
// import java.util.List;

// public static final String TITLE = "title"; //$NON-NLS-1$
pub const TITLE: String = String::from("title");
// public static final String FILE = "fileName"; //$NON-NLS-1$
pub const FILE: String = String::from("fileName");

 /**
  * Places an entry in the <code>Help</code> menu.  Selecting the entry
  * displays a window with a stored image on it.  Good for a splash
  * screen or an "about" screen.
  */
// public class AboutScreen extends AbstractConfigurable {
// class AboutScreen:
pub struct AboutScreen {
  // protected ImageOp op;
  // protected Image image;
  // protected String title;
  // protected String fileName;
  // protected Action launch;
  pub op: ImageOp,
  pub image: Image,
  pub title: String,
  pub file_name: String,
  pub launch: Action
}

impl AboutScreen {
  // public AboutScreen() {
  //   launch = new AbstractAction() {
  //     private static final long serialVersionUID = 1L;
  //
  //     @Override
  //     public void actionPerformed(ActionEvent e) {
  //       launch();
  //     }
  //   };
  // }
  pub fn new() -> Self {
    Self {
      launch: AbstractAction::new(),
    }
  }

  // public AboutScreen(ImageOp op) {
  //   this();
  //   if (op == null) throw new IllegalArgumentException();
  //   this.op = op;
  // // }
  pub fn new2(op: &ImageOp) -> Self {
    Self {

    }
  }

  pub fn action_performed(&mut self, e: &ActionEvent) {
    self.launch()
  }

    // public void launch() {
    // if (op == null) return;

    // final GameModule g = GameModule.getGameModule();
    // if (g == null) return;

    // final StringBuilder sb = new StringBuilder("<html><center>"); //NON-NLS

    // sb.append(
    //   Resources::get_string("AboutScreen.module_version",  //$NON-NLS-1$
    //     g.get_localized_game_name(), g.get_game_version()));

    // final String o1 = g.get_module_other_1();
    // if ((o1 != null) && !o1.isEmpty()) {
      // sb.append(" - ");
      // sb.append(o1);
    // }
    // final String o2 = g.get_module_other_2();
    // if ((o2 != null) && !o2.isEmpty()) {
      // sb.append(" - ");
      // sb.append(o2);
    // }

    // for (final ModuleExtension ext : g.get_components_of(ModuleExtension.class)) {
      // sb.append("<br/>").append(//NON-NLS
        // Resources::get_string("AboutScreen.extension_version",  //$NON-NLS-1$
        //   ext.get_name(), ext.get_version()));
    // }

    // sb.append("<br/>") //NON-NLS
      // .append(
      //   Resources::get_string(
      //     "AboutScreen.vassal_version",  //$NON-NLS-1$
      //     Info.get_version()
      //   )
      // )
      // .append("</center></html>"); //NON-NLS
  // }

  pub fn launch(&mut self) {
      if self.op.is_none() {
        return;
      }

      let mut g: Option<GameModule> = GameModule::get_game_module();
      if g.is_none {
        return;
      }

      let mut sb: String = String::from("<html><center>");
      sb += Resources::getString("AboutScreen.module_version", g.get_localized_game_name(), g.get_game_version());

      let mut o1: Option<String> = g.get_module_other_1();
      if o1.is_some() && o1.unwrap().is_empty() == false {
        sb += " - ";
        sb += o1.unwrap().as_str();
      }

      let mut o2: Option<String> = g.get_module_other_2();
      if o2.is_some() && !o2.unwrap().is_empty() {
        sb += " - ";
        sb += o2.unwrap().as_str();
      }

      for ext in g.get_components_of(ModuleExtension) {
        sb += "<br/>";
        sb += Resources::get_string("AboutScreen.extension_version", ext.get_name(), ext.get_version());
      }

      sb += "<br/>";
      sb += Resources::get_string("AboutScreen.vassal_version", Info.get_version());
      sb += "</center></html>";

  }

  // public static String getConfigureTypeName() {
  //   return Resources::get_string("Editor.AboutScreen.component_type");
  // }
  pub fn get_configure_type_name(&mut self) {
    return Resources::get_string("Editor.AboutScreen.component_type");
  }

  // /**
  //  * The attributes of an AboutScreen are:
  //  *
  //  * <code>TITLE</code> the text of the menu entry in the Help menu
  //  * <code>FILE</code> the name of an image file in the @link
  //  * DataArchive.  The image is displayed when the menu item is
  //  * selected
  //  */
  // @Override
  // public String[] getAttributeNames() {
  //   return new String[]{
  //     TITLE,
  //     FILE
  //   };
  // }
  pub fn get_attribute_names(&mut self) -> Vec<String> {
    let out: Vec<String> = Vec::from([TITLE, FILE]);
    out
  }

  // @Override
  // public String[] getAttributeDescriptions() {
  //   return new String[]{
  //     Resources::get_string("Editor.menu_command"),
  //     Resources::get_string("Editor.AboutScreen.image")
  //   };
  // }
  pub fn get_attribute_descriptions(&mut self) ->  Vec<String> {
    let out: Vec<String> = Vec::from([
      Resources::get_string("Editor.menu_command"),
      Resources::get_string("Editor.AboutScreen.image")
    ]);
    out
  }

  // @Override
  // public Class<?>[] getAttributeTypes() {
  //   return new Class<?>[]{
  //     String.class,
  //     Image.class
  //   };
  // }
  pub fn get_attribute_types<T>(&mut self) -> Vec<T> {
    todo!()
  }

  // @Override
  // public String getAttributeValueString(String key) {
  //   if (TITLE.equals(key)) {
  //     return title;
  //   }
  //   else if (FILE.equals(key)) {
  //     return fileName;
  //   }
  //   return null;
  // }
  pub fn get_attribute_value_string(&mut self, key: &String) -> Option<String> {
    if TITLE == key {
      return Some(self.title);
    } else if FILE == key {
      return Some(self.file_name);
    }
    return None
  }

  // @Override
  // public void setAttribute(String key, Object val) {
    pub fn set_attribute<T>(&mut self, key: &String, val: T)
    {
  //   if (TITLE.equals(key)) {
  //     title = (String) val;

  //     // don't permit "About VASSAL"
  //     if (title != null && title.equals(Resources::get_string("AboutScreen.about_vassal"))) {
  //       title = Resources::get_string("Documentation.about_module");
  //     }

  //     setConfigureName(title);
  //     launch.putValue(Action.NAME, title);
  //   }
  //   else if (FILE.equals(key)) {
  //     if (val instanceof File) {
  //       val = ((File) val).get_name();
  //     }
  //     fileName = (String) val;

  //     op = null;
  //     if (fileName != null) {
  //       fileName = fileName.trim();
  //       if (fileName.length() > 0) {
  //         op = Op.load(fileName);

  //         final Image img = op.getImage();
  //         if (img != null) {
  //           GameModule.getGameModule()
  //                     .getWizardSupport()
  //                     .setBackgroundImage(op.getImage());
  //         }
  //         else {
  //           op = null;
  //         }
  //       }
  //     }
  //   }
  // }
    }

}

impl AbstractConfigurable for AboutScreen {

}



  


  


  

  

  

  

  



  

  @Override
  public Class<?>[] getAllowableConfigureComponents() {
    return new Class<?>[0];
  }

  @Override
  public void removeFrom(Buildable b) {
    MenuManager.getInstance().removeAction("Documentation.about_module");
  }

  /**
   * Expects to be added to a {@link Documentation}.  Adds an entry
   * to the <code>Help</code> menu
   */
  @Override
  public void addTo(Buildable b) {
    MenuManager.getInstance().addAction("Documentation.about_module", launch);
  }

  @Override
  public HelpFile getHelpFile() {
    return HelpFile.getReferenceManualPage("HelpMenu.html", "AboutScreen"); //$NON-NLS-1$ //$NON-NLS-2$
  }

  /**
   * {@link VASSAL.search.SearchTarget}
   * @return a list of any Message Format strings referenced in the Configurable, if any (for search)
   */
  @Override
  public List<String> getFormattedStringList() {
    return List.of(title);
  }

  @Override
  public void addLocalImageNames(Collection<String> s) {
    if (fileName != null) s.add(fileName);
  }
}
