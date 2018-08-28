
import React, { Component } from 'react'
import { Link } from 'react-router-dom'
import { Menu } from 'semantic-ui-react'

export class SiteMenu extends Component {
  constructor (props) {
    super(props)
    this.state = { activeItem: this.props.location.pathname }
  }

  render () {
    const { activeItem } = this.state
    const pages = [
      ['Search', '/'],
      ['About', '/about'],
      ['To do', '/todo']
    ]

    return (
      <div>
        <Menu pointing secondary>
          {pages.map((page, i) =>
            <Menu.Item
              key={i}
              as={Link}
              to={page[1]}
              name={page[0]}
              active={activeItem === page[1]}
            />
          )}
          <Menu.Item name='Blog' onClick={() => window.location.replace('http://brahmlower.io/tag/gifzone.html')} />
        </Menu>
      </div>
    )
  }
}

export default SiteMenu
