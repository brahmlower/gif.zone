import React, { Component } from 'react';
import { Switch, Route, Link, BrowserRouter } from "react-router-dom";
import { Container, Menu } from 'semantic-ui-react';

import { PageSearch } from './pages/Search.js'
import { PageAbout } from './pages/About.js'
import { PageTodo } from './pages/Todo.js'
import './App.css';

class App extends Component {
  render() {
    return (
      <BrowserRouter>
        <div className="App">
          <Container>
            <h1> Gif zone </h1>
            <SiteMenu />
            <Switch>
              <Route exact path="/" component={PageSearch} />
              <Route exact path="/about" component={PageAbout} />
              <Route exact path="/todo" component={PageTodo} />
            </Switch>
          </Container>
        </div>
      </BrowserRouter>
    );
  }
}

export class SiteMenu extends Component {
  state = { activeItem: 'Search' }

  handleItemClick = (e, { name }) => this.setState({ activeItem: name })

  render() {
    const { activeItem } = this.state
    const pages = [
      ['Search', '/'],
      ['About', '/about'],
      ['To do', '/todo']
    ]

    return (
      <div>
        <Menu pointing secondary>
          {pages.map((page) =>
            <Menu.Item
              as={Link}
              to={page[1]}
              name={page[0]}
              active={activeItem === page[0]}
              onClick={this.handleItemClick}
            />
          )}
          <Menu.Item name="Blog" onClick={() => window.location.replace("http://brahmlower.io/tag/gifzone.html")} />
        </Menu>
      </div>
    )
  }
}

export default App;
