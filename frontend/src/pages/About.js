import React, { Component } from 'react'
import { SiteMenu } from '../components/Menu'

class PageAbout extends Component {
  render () {
    return (
      <div>
        <SiteMenu location={ this.props.location }/>
        <p> Gif Zone is meant to be a collection of curated, quality gifs that are easily search and linked, with minimal advertising and branding. Other big-name gif sites are too resource intensive and distract from their core purpose. This is a personal project I'm developing in my spare time, so feature implementation may be slow. Development updates will be posted every month or so on my blog, and To Do items will be listed on the todo tab once the project repo is open sourced.</p>
      </div>
    )
  }
}

export { PageAbout }
