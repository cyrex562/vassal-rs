/*
 *
 * Copyright (c) 2000-2003 by Rodney Kinney
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License (LGPL) as published by the Free Software Foundation.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library; if not, copies are available
 * at http://www.opensource.org.
 */
# package VASSAL.build;

import org.w3c.dom.Attr;
import org.w3c.dom.Document;
import org.w3c.dom.Element;
import org.w3c.dom.NamedNodeMap;

import VASSAL.configure.AutoConfigurer;
import VASSAL.configure.VisibilityCondition;
import VASSAL.i18n.Localization;

/**
 * A class that implements AutoConfigurable can use the {@link AutoConfigurer}
 * class to automatically build a property editor.
 */
public interface AutoConfigurable extends Configurable {
  /**
   * @return an array of Strings giving all attributes of this Buildable
   * component that will be written to/read from an XML element
   */
  String[] getAttributeNames();

  /**
   * Called by the {@link #build} method, where <code>value</code> is the
   * String value read by the XML attribute.
   * Can also be called with Object value to set the attribute.
   */
  @Override
  void setAttribute(String key, Object value);

  /**
   * Called by the {@link #getBuildElement} method to write the
   * attributes into an XML element
   */
  @Override
  String getAttributeValueString(String key);

  /**
   * Return an array of Strings describing the attributes
   * of this object.  These strings are used as prompts in
   * the Properties window for this object.
   */
  String[] getAttributeDescriptions();

  /**
   * Return the Class for the attributes of this object.
   * Valid classes are:  String, Integer, Double, Boolean, Image,
   * File, Color, and KeyStroke
   */
  Class<?>[] getAttributeTypes();

  /**
   * Because attributes are not always applicable in all cases, this method returns an interface
   * to determine when the controls for specifying the named attribute should be visible.
   * @param name
   * @return null if the attribute controls should always be visible;
   */
  VisibilityCondition getAttributeVisibility(String name);

  class Util {
    public static void buildAttributes(Element e, AutoConfigurable parent) {
      if (e != null) {
        final NamedNodeMap n = e.getAttributes();
        for (int i = 0; i < n.getLength(); ++i) {
          final Attr att = (Attr) n.item(i);
          parent.setAttribute(att.get_name(), att.getValue());
          Localization.getInstance().saveTranslatableAttribute(parent, att.get_name(), att.getValue());
        }
      }
    }

    public static Element getBuildElement(Document doc, AutoConfigurable parent) {
      final Element el = doc.createElement(parent.getClass().get_name());
      final String[] names = parent.getAttributeNames();
      for (final String name : names) {
        final String val = parent.getAttributeValueString(name);
        if (val != null) {
          el.setAttribute(name, val);
        }
      }
      return el;
    }
  }
}
