import React, { Component } from 'react'
import { Switch, Route, BrowserRouter } from 'react-router-dom'
import { Container } from 'semantic-ui-react'

import { PageSearch } from './pages/Search.js'
import { PageAbout } from './pages/About.js'
import { PageTodo } from './pages/Todo.js'
import './App.css'

class App extends Component {
  render () {
    return (
      <BrowserRouter>
        <div className='App'>
          <Container>
            <h1> Gif zone </h1>
            <Switch>
              <Route exact path='/' component={PageSearch} />
              <Route exact path='/about' component={PageAbout} />
              <Route exact path='/todo' component={PageTodo} />
            </Switch>
          </Container>
        </div>
      </BrowserRouter>
    )
  }
}

export default App
