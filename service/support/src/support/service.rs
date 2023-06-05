use tonic::{Request, Response, Status};

use crate::support::interactor::SupportInteractor;
use crate::support::pb::{CreateTicketRequest, CreateTicketResponse, DeleteTicketRequest, DeleteTicketResponse, GetTicketByIdRequest, GetTicketByIdResponse, GetTicketsRequest, GetTicketsResponse, UpdateTicketRequest, UpdateTicketResponse};
use crate::support::pb::support_service_server::SupportService;

pub struct SupportServiceImpl {
    interactor: Box<dyn SupportInteractor + Send + Sync>,
}

impl SupportServiceImpl {
    pub fn new(interactor: Box<dyn SupportInteractor + Send + Sync>) -> impl SupportService {
        SupportServiceImpl { interactor }
    }
}

#[tonic::async_trait]
impl SupportService for SupportServiceImpl {
    async fn get_tickets(&self, request: Request<GetTicketsRequest>) -> Result<Response<GetTicketsResponse>, Status> {
        let GetTicketsRequest { user_id, skip, limit } = request.into_inner();
        if user_id.is_empty() {
            return status::Status::invalid_arguments(vec!["user_id"]);
        }

        match self.interactor.get_tickets(&user_id, skip, limit).await {
            Ok(tickets) => Ok(Response::new(GetTicketsResponse { tickets: tickets.into_iter().map(|ticket| ticket.into()).collect() })),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn get_ticket_by_id(&self, request: Request<GetTicketByIdRequest>) -> Result<Response<GetTicketByIdResponse>, Status> {
        let GetTicketByIdRequest { id } = request.into_inner();
        if id.is_empty() {
            return status::Status::invalid_arguments(vec!["id"]);
        }

        match self.interactor.get_ticket_by_id(&id).await {
            Ok(ticket) => Ok(Response::new(GetTicketByIdResponse { ticket: Some(ticket.into()) })),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn create_ticket(&self, request: Request<CreateTicketRequest>) -> Result<Response<CreateTicketResponse>, Status> {
        let CreateTicketRequest { user_id, topic, description } = request.into_inner();
        if user_id.is_empty() || topic.is_empty() || description.is_empty() {
            return status::Status::invalid_arguments(vec!["user_id", "topic", "description"]);
        }

        match self.interactor.create_ticket(&user_id, &topic, &description).await {
            Ok(status) => Ok(Response::new(CreateTicketResponse { status: status as i32 })),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn update_ticket(&self, request: Request<UpdateTicketRequest>) -> Result<Response<UpdateTicketResponse>, Status> {
        let UpdateTicketRequest { ticket_id, description } = request.into_inner();
        if ticket_id.is_empty() || description.is_empty() {
            return status::Status::invalid_arguments(vec!["ticket_id", "description"]);
        }

        match self.interactor.update_ticket(&ticket_id, &description).await {
            Ok(status) => Ok(Response::new(UpdateTicketResponse { status: status as i32 })),
            Err(error) => status::Status::internal(error)
        }
    }

    async fn delete_ticket(&self, request: Request<DeleteTicketRequest>) -> Result<Response<DeleteTicketResponse>, Status> {
        let DeleteTicketRequest { ticket_id } = request.into_inner();
        if ticket_id.is_empty() {
            return status::Status::invalid_arguments(vec!["ticket_id"]);
        }

        match self.interactor.delete_ticket(&ticket_id).await {
            Ok(ticket_id) => Ok(Response::new(DeleteTicketResponse { ticket_id })),
            Err(error) => status::Status::internal(error)
        }
    }
}