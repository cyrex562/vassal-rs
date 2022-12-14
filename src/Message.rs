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
# package VASSAL.chat.messageboard;

import java.util.Date;

/**
 * A message in a message board
 * @author rkinney
 *
 */
public class Message {
  private final String sender;
  private final String text;
  private final Date creationTime;

  public Message(String sender, String text) {
    this(sender, text, new Date());
  }

  public Message(String sender, String text, Date created) {
    this.sender = sender;
    this.text = text;
    creationTime = created;
  }

  public String getSender() {
    return sender;
  }

  public String getText() {
    return text;
  }

  public Date getDate() {
    return creationTime;
  }
}
