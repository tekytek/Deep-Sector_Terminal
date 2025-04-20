use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};

use crate::{
    game::Game,
    models::market::{OrderStatus, OrderType},
    ui::colors,
};

pub fn draw_orders_screen<B: Backend>(f: &mut Frame<B>, game: &Game, area: Rect) {
    // Create the layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title + tabs
            Constraint::Min(10),    // Orders list
            Constraint::Length(7),  // Order details
            Constraint::Length(3),  // Controls
        ])
        .split(area);

    // Draw title
    let title = "Trade Orders Management";
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::PRIMARY));
    
    let title_text = Paragraph::new(Spans::from(Span::styled(
        title,
        Style::default().fg(colors::PRIMARY).add_modifier(Modifier::BOLD),
    )))
    .block(title_block);
    
    f.render_widget(title_text, chunks[0]);
    
    // Create tabs for switching between active and completed orders
    let titles = vec!["Active Orders", "Completed Orders"];
    let tabs_index = if game.orders_view_active { 0 } else { 1 };
    
    let tabs = Tabs::new(titles.into_iter().map(Spans::from).collect())
        .select(tabs_index)
        .style(Style::default().fg(colors::DIM))
        .highlight_style(Style::default().fg(colors::PRIMARY).add_modifier(Modifier::BOLD))
        .divider("|");
    
    f.render_widget(tabs, chunks[0]);
    
    // Get the appropriate orders to display
    let orders = if game.orders_view_active {
        game.trading_system.get_active_orders(&game.player)
    } else {
        game.trading_system.get_completed_orders(&game.player)
    };
    
    // Order list
    let orders_items: Vec<ListItem> = if orders.is_empty() {
        vec![ListItem::new(Spans::from(Span::styled(
            "No orders found",
            Style::default().fg(colors::DIM),
        )))]
    } else {
        orders
            .iter()
            .enumerate()
            .map(|(i, order)| {
                let is_selected = game.trading_system.get_selected_order_index() == Some(i);
                let style = if is_selected {
                    Style::default().fg(colors::PRIMARY).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(colors::DEFAULT_TEXT)
                };
                
                let order_type_text = match order.order_type {
                    OrderType::Buy => "BUY",
                    OrderType::Sell => "SELL",
                };
                
                let order_status_text = match order.status {
                    OrderStatus::Active => "ACTIVE",
                    OrderStatus::Completed => "COMPLETED",
                    OrderStatus::Cancelled => "CANCELLED",
                    OrderStatus::Failed => "FAILED",
                    OrderStatus::Expired => "EXPIRED",
                };
                
                let order_status_color = match order.status {
                    OrderStatus::Active => colors::PRIMARY,
                    OrderStatus::Completed => colors::SUCCESS,
                    OrderStatus::Cancelled => colors::WARNING,
                    OrderStatus::Failed => colors::DANGER,
                    OrderStatus::Expired => colors::DIM,
                };
                
                let content = Spans::from(vec![
                    Span::styled(
                        format!("{:<5}", order_type_text),
                        Style::default().fg(match order.order_type {
                            OrderType::Buy => colors::INFO,
                            OrderType::Sell => colors::SUCCESS,
                        }),
                    ),
                    Span::raw(" | "),
                    Span::styled(
                        format!("{:<15}", order.item_name),
                        style,
                    ),
                    Span::raw(" | "),
                    Span::styled(
                        format!("Qty: {:<5}", order.quantity),
                        style,
                    ),
                    Span::raw(" | "),
                    Span::styled(
                        format!("Price: {:<7}", order.target_price),
                        style,
                    ),
                    Span::raw(" | "),
                    Span::styled(
                        format!("{:<10}", order_status_text),
                        Style::default().fg(order_status_color),
                    ),
                ]);
                
                ListItem::new(content)
            })
            .collect()
    };
    
    let orders_list = List::new(orders_items)
        .block(Block::default()
            .title(if game.orders_view_active { "Active Orders" } else { "Completed Orders" })
            .borders(Borders::ALL)
            .border_style(Style::default().fg(colors::DIM)))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(colors::PRIMARY))
        .highlight_symbol("> ");
    
    f.render_widget(orders_list, chunks[1]);
    
    // Order details
    let details_block = Block::default()
        .title("Order Details")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::DIM));
    
    let details_text = if let Some(index) = game.trading_system.get_selected_order_index() {
        if let Some(order) = orders.get(index) {
            let status_text = match order.status {
                OrderStatus::Active => "Active",
                OrderStatus::Completed => "Completed",
                OrderStatus::Cancelled => "Cancelled",
                OrderStatus::Failed => "Failed",
                OrderStatus::Expired => "Expired",
            };
            
            let status_color = match order.status {
                OrderStatus::Active => colors::PRIMARY,
                OrderStatus::Completed => colors::SUCCESS,
                OrderStatus::Cancelled => colors::WARNING,
                OrderStatus::Failed => colors::DANGER,
                OrderStatus::Expired => colors::DIM,
            };
            
            let order_type = match order.order_type {
                OrderType::Buy => "Buy",
                OrderType::Sell => "Sell",
            };
            
            let order_type_color = match order.order_type {
                OrderType::Buy => colors::INFO,
                OrderType::Sell => colors::SUCCESS,
            };
            
            vec![
                Spans::from(vec![
                    Span::styled("Type: ", Style::default().fg(colors::DIM)),
                    Span::styled(order_type, Style::default().fg(order_type_color)),
                    Span::raw("  |  "),
                    Span::styled("Status: ", Style::default().fg(colors::DIM)),
                    Span::styled(status_text, Style::default().fg(status_color)),
                ]),
                Spans::from(vec![
                    Span::styled("Item: ", Style::default().fg(colors::DIM)),
                    Span::styled(&order.item_name, Style::default().fg(colors::DEFAULT_TEXT)),
                ]),
                Spans::from(vec![
                    Span::styled("Quantity: ", Style::default().fg(colors::DIM)),
                    Span::styled(order.quantity.to_string(), Style::default().fg(colors::DEFAULT_TEXT)),
                    Span::raw("  |  "),
                    Span::styled("Target Price: ", Style::default().fg(colors::DIM)),
                    Span::styled(order.target_price.to_string(), Style::default().fg(colors::DEFAULT_TEXT)),
                ]),
                Spans::from(vec![
                    Span::styled("Total Value: ", Style::default().fg(colors::DIM)),
                    Span::styled(
                        format!("{}", order.quantity as u64 * order.target_price as u64),
                        Style::default().fg(colors::DEFAULT_TEXT),
                    ),
                ]),
                Spans::from(vec![
                    Span::styled("Notes: ", Style::default().fg(colors::DIM)),
                    Span::styled(&order.notes, Style::default().fg(colors::DEFAULT_TEXT)),
                ]),
            ]
        } else {
            vec![Spans::from(Span::styled(
                "No order selected",
                Style::default().fg(colors::DIM),
            ))]
        }
    } else {
        vec![Spans::from(Span::styled(
            "No order selected",
            Style::default().fg(colors::DIM),
        ))]
    };
    
    let details = Paragraph::new(details_text)
        .block(details_block);
    
    f.render_widget(details, chunks[2]);
    
    // Controls
    let controls_block = Block::default()
        .title("Controls")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::DIM));
    
    let controls_text = if game.orders_view_active {
        vec![
            Spans::from(vec![
                Span::styled("↑/↓", Style::default().fg(colors::PRIMARY)),
                Span::raw(": Navigate  "),
                Span::styled("[B]", Style::default().fg(colors::PRIMARY)),
                Span::raw("uy  "),
                Span::styled("[S]", Style::default().fg(colors::PRIMARY)),
                Span::raw("ell  "),
                Span::styled("[C]", Style::default().fg(colors::PRIMARY)),
                Span::raw("ancel  "),
                Span::styled("[Tab]", Style::default().fg(colors::PRIMARY)),
                Span::raw(": Show Completed  "),
                Span::styled("[M]", Style::default().fg(colors::PRIMARY)),
                Span::raw("enu"),
            ]),
        ]
    } else {
        vec![
            Spans::from(vec![
                Span::styled("↑/↓", Style::default().fg(colors::PRIMARY)),
                Span::raw(": Navigate  "),
                Span::styled("[Tab]", Style::default().fg(colors::PRIMARY)),
                Span::raw(": Show Active  "),
                Span::styled("[M]", Style::default().fg(colors::PRIMARY)),
                Span::raw("enu"),
            ]),
        ]
    };
    
    let controls = Paragraph::new(controls_text)
        .block(controls_block);
    
    f.render_widget(controls, chunks[3]);
}