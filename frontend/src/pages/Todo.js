import React, { Component } from 'react'
import { Segment } from 'semantic-ui-react';
import { SiteMenu } from '../components/Menu'

class PageTodo extends Component {
  constructor (props) {
    super(props)
    this.state = {
      issues: []
    }
    this.issueList = this.issueList.bind(this)
  }

  componentDidMount () {
    // This was designed around consuming the github API, but the api really only needs to support
    // the 'labels' url parameter, and return an object with the properties: html_url, title, and body
    const issue_url = 'https://api.github.com/repos/bplower/ansible-factorio/issues?labels=enhancement'
    fetch(issue_url)
    .then(res => res.json())
    .then(
      (result) => {
        this.setState({
          issues: result
        })
      },
      (error) => {
        console.log('api loading error!')
        console.log(error)
        this.setState({
          isLoaded: true,
          error
        })
      }
    )
  }

  issueList() {
    return this.state.issues.map((issue, i) => {
      return (
        <Segment key={i} padded>
          <a href={ issue.html_url }>
            <h3> { issue.title } <i class="external alternate icon"></i></h3>
          </a>
          <p> { issue.body } </p>
        </Segment>
      )
    })
  }

  render () {
    return (
      <div>
        <SiteMenu location={ this.props.location }/>
        <p> Gif.zone is a work in progress! Here is a to do list of features, fixes and improvements I have planned. These are all tracked on github, so you may comment on to do items if you have any concerns.</p>
        <Segment.Group children={this.issueList()} />
      </div>
    )
  }
}

export { PageTodo }
